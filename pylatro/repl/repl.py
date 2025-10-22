"""Basic REPL for manual Balatro play using pylatro bindings."""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import dataclass
from typing import List, Optional

# Add the parent directory to the path to find pylatro
import os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

import pylatro


def _format_money(amount: float) -> str:
    return f"${amount:.0f}"


def _format_stage(stage) -> str:  # pragma: no cover - simple helper
    try:
        return str(stage)
    except Exception:  # noqa: BLE001 - defensive against PyO3 repr issues
        return "<stage>"


@dataclass
class GameSummary:
    score: float
    required_score: float
    money: float
    ante: float
    stage: str
    plays: float
    discards: float
    round: float

    @classmethod
    def from_engine(cls, engine: pylatro.GameEngine) -> "GameSummary":
        state = engine.state
        return cls(
            score=state.score,
            required_score=state.required_score,
            money=state.money,
            ante=state.ante,
            stage=_format_stage(state.stage),
            plays=state.plays,
            discards=state.discards,
            round=state.round,
        )

    def lines(self) -> List[str]:
        return [
            f"Stage: {self.stage} (Ante {self.ante}, Round {self.round})",
            f"Score: {self.score:.0f} / {self.required_score:.0f}",
            f"Bankroll: {_format_money(self.money)}",
            f"Plays: {self.plays:.0f}, Discards: {self.discards:.0f}",
        ]


def _print_header(engine: pylatro.GameEngine) -> None:
    print("\n=== Balatro Game State ===")
    for line in GameSummary.from_engine(engine).lines():
        print(line)
    print()
    _list_actions(engine)


def _format_card(engine: pylatro.GameEngine, card_index: int) -> str:
    """Format a card using the same method as action names to get colors."""
    try:
        # Find the action that corresponds to this card
        space = engine.gen_action_space()
        for i, valid in enumerate(space):
            if valid:
                action_name = engine.get_action_name(i)
                if action_name.startswith("SelectCard:"):
                    # Extract the card part after "SelectCard: "
                    card_part = action_name.split(": ", 1)[1]
                    # Check if this is the card we want by comparing with the card at card_index
                    if i == card_index:
                        return card_part
        # Fallback to basic string representation
        return str(engine.state.available[card_index])
    except Exception:
        return str(engine.state.available[card_index])


def _describe_hand(engine: pylatro.GameEngine) -> None:
    state = engine.state
    print("Hand:")
    for idx, card in enumerate(state.available):
        formatted_card = _format_card(engine, idx)
        print(f"  [{idx}] {formatted_card}")
    print()


def _describe_jokers(engine: pylatro.GameEngine) -> None:
    state = engine.state
    print("Jokers:")
    if not state.joker_ids:
        print("  (none)")
        print()
        return
    for joker_id in state.joker_ids:
        info = engine.get_joker_info(joker_id)
        name = info.name if info else str(joker_id)
        print(f"  - {name}")
    print()


def _list_actions(engine: pylatro.GameEngine) -> List[str]:
    space = engine.gen_action_space()
    lines = []
    for index, valid in enumerate(space):
        if not valid:
            continue
        try:
            name = engine.get_action_name(index)
        except Exception:  # noqa: BLE001 - bail gracefully on PyO3 errors
            name = "<unknown action>"
        lines.append(f"[{index}] {name}")
    print("Available actions:")
    for line in lines:
        print(f"  {line}")
    print()
    return lines


def _load_config(path: Optional[str]) -> pylatro.Config:
    if path is None:
        return pylatro.Config()

    with open(path, "r", encoding="utf-8") as fh:
        data = json.load(fh)

    config = pylatro.Config()
    for key, value in data.items():
        if not hasattr(config, key):
            print(f"Warning: unknown config key '{key}'", file=sys.stderr)
            continue
        setattr(config, key, value)
    return config


def _handle_play(engine: pylatro.GameEngine, arg: str) -> None:
    try:
        index = int(arg)
    except ValueError:  # noqa: PERF203 - readability
        print("Expected integer index. Use 'actions' to list valid indices.")
        return

    try:
        engine.handle_action_index(index)
    except Exception as exc:  # noqa: BLE001 - surface PyO3 GameError
        print(f"Failed to execute action {index}: {exc}")


def _prompt() -> str:
    try:
        return input("balatro> ").strip()
    except EOFError:  # noqa: PERF203 - clarity
        return "quit"


def run(args: Optional[List[str]] = None) -> None:
    parser = argparse.ArgumentParser(description="Manual Balatro REPL")
    parser.add_argument(
        "--config",
        type=str,
        help="Path to JSON config file to initialize the game",
    )
    parser.add_argument(
        "--seed",
        type=int,
        help="Optional RNG seed for deterministic runs",
    )
    parsed = parser.parse_args(args=args)

    config = _load_config(parsed.config)
    # Note: Config doesn't have a seed attribute in the current API
    # if parsed.seed is not None:
    #     config.seed = parsed.seed

    engine = pylatro.GameEngine(config)
    print("Starting Balatro manual REPL. Type 'help' for commands.\n")
    _print_header(engine)

    while True:
        if engine.is_over:
            result = "win" if engine.is_win else "loss"
            print(f"Game over! Result: {result}. Final score: {engine.state.score:.0f}")
            break

        command = _prompt()
        if not command:
            continue

        cmd, *rest = command.split(maxsplit=1)
        argument = rest[0] if rest else ""

        if cmd in {"quit", "exit"}:
            print("Exiting Balatro REPL.")
            break
        if cmd == "help":
            print(
                "Commands:\n"
                "  <number>       Execute action by index (e.g., '78')\n"
                "  help          Show this message\n"
                "  state         Display core game summary\n"
                "  hand          List current hand cards\n"
                "  jokers        Show current jokers\n"
                "  actions       List available actions\n"
                "  restart       Restart game with same config\n"
                "  quit/exit     Leave the REPL\n"
            )
            continue
        if cmd == "state":
            _print_header(engine)
            continue
        if cmd == "hand":
            _describe_hand(engine)
            continue
        if cmd == "jokers":
            _describe_jokers(engine)
            continue
        if cmd == "actions":
            _list_actions(engine)
            continue
        if cmd == "restart":
            engine = pylatro.GameEngine(config)
            print("Game restarted.\n")
            _print_header(engine)
            continue

        # Try to parse as action index
        try:
            action_index = int(cmd)
            _handle_play(engine, cmd)
            if not engine.is_over:
                _print_header(engine)
            continue
        except ValueError:
            pass  # Not a number, continue to error message

        print(f"Unknown command: {cmd}. Type 'help' for options.")


def main() -> None:
    run()


if __name__ == "__main__":  # pragma: no cover - CLI guard
    main()

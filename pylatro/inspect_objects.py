#!/usr/bin/env python3
"""
Pylatro Object Inspector

This script helps you explore and understand the pylatro objects
since you can't "command-click" into them (they're Rust bindings).
"""

import pylatro
import inspect
from typing import Any, Dict, List


def inspect_object(obj: Any, name: str = "object", max_depth: int = 2, current_depth: int = 0) -> None:
    """Recursively inspect an object and its attributes."""
    if current_depth > max_depth:
        return

    indent = "  " * current_depth
    print(f"{indent}=== {name} ===")
    print(f"{indent}Type: {type(obj)}")
    print(f"{indent}Module: {type(obj).__module__}")

    # Try to get string representation
    try:
        str_repr = str(obj)
        if len(str_repr) > 100:
            str_repr = str_repr[:100] + "..."
        print(f"{indent}Value: {str_repr}")
    except:
        print(f"{indent}Value: <unable to convert to string>")

    # Get attributes
    attrs = [attr for attr in dir(obj) if not attr.startswith('_')]
    if attrs:
        print(f"{indent}Attributes: {', '.join(attrs[:10])}")
        if len(attrs) > 10:
            print(f"{indent}  ... and {len(attrs) - 10} more")

    # Inspect important attributes
    important_attrs = ['stage', 'round', 'score', 'money', 'ante', 'plays', 'discards']
    for attr in important_attrs:
        if hasattr(obj, attr) and current_depth < max_depth:
            try:
                value = getattr(obj, attr)
                inspect_object(value, f"{name}.{attr}", max_depth, current_depth + 1)
            except:
                pass


def explore_game_engine():
    """Explore a GameEngine object in detail."""
    print("🎮 Creating GameEngine...")
    game = pylatro.GameEngine()

    print("\n" + "="*60)
    print("GAME ENGINE INSPECTION")
    print("="*60)

    inspect_object(game, "GameEngine", max_depth=1)

    print("\n" + "="*60)
    print("GAME STATE INSPECTION")
    print("="*60)

    state = game.state
    inspect_object(state, "GameState", max_depth=2)

    print("\n" + "="*60)
    print("ACTION SPACE ANALYSIS")
    print("="*60)

    action_space = game.gen_action_space()
    print(f"Action space length: {len(action_space)}")
    print(f"Valid actions: {sum(action_space)} out of {len(action_space)}")
    print(f"Action space type: {type(action_space)}")

    # Show valid action indices
    valid_indices = [i for i, valid in enumerate(action_space) if valid]
    print(f"Valid action indices: {valid_indices}")

    print("\n" + "="*60)
    print("AVAILABLE ACTIONS")
    print("="*60)

    actions = game.gen_actions()
    print(f"Number of available actions: {len(actions)}")

    for i, action in enumerate(actions):
        print(f"\nAction {i}:")
        inspect_object(action, f"Action_{i}", max_depth=1)

        # Try to get action name
        try:
            action_name = game.get_action_name(valid_indices[i])
            print(f"  Action name: {action_name}")
        except:
            pass


def explore_after_action():
    """Explore the game state after taking an action."""
    print("\n" + "="*60)
    print("AFTER TAKING AN ACTION")
    print("="*60)

    game = pylatro.GameEngine()

    print("Before action:")
    print(f"  Stage: {game.state.stage}")
    print(f"  Score: {game.state.score}")
    print(f"  Money: {game.state.money}")

    # Take first available action
    action_space = game.gen_action_space()
    valid_indices = [i for i, valid in enumerate(action_space) if valid]

    if valid_indices:
        action_index = valid_indices[0]
        print(f"\nTaking action index {action_index}...")
        game.handle_action_index(action_index)

        print("After action:")
        print(f"  Stage: {game.state.stage}")
        print(f"  Score: {game.state.score}")
        print(f"  Money: {game.state.money}")
        print(f"  Round: {game.state.round}")
        print(f"  Plays: {game.state.plays}")
        print(f"  Discards: {game.state.discards}")


def explore_joker_system():
    """Explore the joker system."""
    print("\n" + "="*60)
    print("JOKER SYSTEM EXPLORATION")
    print("="*60)

    game = pylatro.GameEngine()

    print("Joker slots:")
    print(f"  Used: {game.state.joker_slots_used}")
    print(f"  Total: {game.state.joker_slots_total}")
    print(f"  Available: {game.state.joker_slots_total - game.state.joker_slots_used}")

    print("\nJoker IDs:")
    print(f"  Current jokers: {game.state.joker_ids}")

    print("\nAvailable jokers:")
    try:
        available_jokers = game.get_available_jokers()
        print(f"  Number available: {len(available_jokers)}")
        if available_jokers:
            print(f"  First few: {available_jokers[:3]}")
    except Exception as e:
        print(f"  Error getting available jokers: {e}")


if __name__ == "__main__":
    print("🔍 Pylatro Object Inspector")
    print("This script helps you understand pylatro objects since you can't 'command-click' into them.")

    try:
        explore_game_engine()
        explore_after_action()
        explore_joker_system()

        print("\n" + "="*60)
        print("SUMMARY")
        print("="*60)
        print("✅ All objects are Rust bindings (PyO3)")
        print("✅ Use this script to explore object structure")
        print("✅ Use debug prints in your code to inspect values")
        print("✅ Use VS Code's debugger with 'Step Over' for Rust calls")

    except Exception as e:
        print(f"❌ Error during inspection: {e}")
        import traceback
        traceback.print_exc()

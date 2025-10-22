import pylatro
import random


def inspect_pylatro_object(obj, name="object", max_items=5):
    """Quick inspection helper for pylatro objects."""
    print(f"\n🔍 {name}:")
    print(f"  Type: {type(obj)}")
    print(f"  Value: {obj}")

    # Get attributes
    attrs = [attr for attr in dir(obj) if not attr.startswith('_')]
    if attrs:
        print(f"  Attributes ({len(attrs)}): {', '.join(attrs[:max_items])}")
        if len(attrs) > max_items:
            print(f"    ... and {len(attrs) - max_items} more")

    return obj


# Test action space (vector) api
def test_action_space():
    game = pylatro.GameEngine()

    # Inspect the game engine
    inspect_pylatro_object(game, "GameEngine")
    inspect_pylatro_object(game.state, "GameState")

    while True:
        if game.is_over:
            break

        # Generate static length action space vector
        action_space = game.gen_action_space()

        # Inspect action space
        print(f"\n📊 Action Space: {len(action_space)} total, {sum(action_space)} valid")
        valid_indices = [i for i, valid in enumerate(action_space) if valid]
        print(f"   Valid indices: {valid_indices}")

        # Vector is masked, invalid actions are 0, valid are 1.
        # We only want to execute valid actions.
        while True:
            index = random.choice(range(len(action_space)))
            if action_space[index] == 1:
                print(f"\n🎯 Executing action index {index}")
                print(f"   Before: {game.state}")

                game.handle_action_index(index)

                print(f"   After:  {game.state}")
                break

    assert game.is_over
    if game.is_win:
        print("🎉 game win!")
    else:
        print("💀 game loss!")
    print(f"Final state: {game.state}")


if __name__ == "__main__":
    test_action_space()

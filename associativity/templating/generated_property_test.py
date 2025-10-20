import random

def check_property(repetitions):
    correct_count = 0
    for _ in range(repetitions):
        x = random.uniform(-100.0, 100.0)
        y = random.uniform(-100.0, 100.0)
        z = random.uniform(-100.0, 100.0)

        result1 = x + (y + z)
        result2 = (x + y) + z

        if abs(result1 - result2) <= 0:
            correct_count += 1

    print(f"Out of {repetitions} repetitions, the property held {correct_count} times ({correct_count / repetitions * 100}%).")

repetitions = 10000
check_property(repetitions)
import random

def check_property(repetitions):
    correct_count = 0
    for _ in range(repetitions):
        x = random.uniform(-500.0, 500.0)
        y = random.uniform(-500.0, 500.0)
        z = random.uniform(-500.0, 500.0)

        result1 = x / (y + 1e-6)
        result2 = (y + 1e-6) / x

        if abs(result1 - result2) <= 0.01:
            correct_count += 1

    print(f"Out of {repetitions} repetitions, the property held {correct_count} times ({correct_count / repetitions * 100}%).")

repetitions = 6000
check_property(repetitions)
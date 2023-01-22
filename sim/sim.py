import argparse
import random
import dataclasses


def main():
    parser = argparse.ArgumentParser(description="simulation")
    parser.add_argument("--num", type=int, default=5)
    parser.add_argument("--iter", type=int, default=1000)
    parser.add_argument("--seed", type=int, default=None)
    args = parser.parse_args()

    result = simulate(args.num, args.iter, args.seed)

    display(args.num, result)


def simulate(num, iterations, seed):
    # initialize random generator
    random.seed(seed)

    points = init_point(num)
    users = range(len(points))
    selection = []
    for i in range(iterations):
        picked = pick(users, points)
        selection.append(picked)
        points[picked] += 1

    return selection, points


def pick(users, points):
    max_point = max(points)
    weights = [max_point - x for x in points]
    if all([x == 0 for x in weights]):
        return random.choice(users)
    return random.choices(users, weights)[0]


def init_point(users, initial_max_point=10):
    return [random.randint(0, initial_max_point) for x in range(users)]


def display(num, result):
    selection, points = result
    count = [selection.count(x) for x in range(num)]
    print(count)
    print(points)


if __name__ == "__main__":
    main()

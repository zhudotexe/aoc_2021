# Python port of my day 6 solution for Antonio
import functools


@functools.lru_cache
def lanternfish(timer: int, days: int) -> int:
    if days == 0:
        return 1
    elif timer == 0:
        return lanternfish(6, days - 1) + lanternfish(8, days - 1)
    return lanternfish(timer - 1, days - 1)


if __name__ == '__main__':
    fish_input = list(map(int, input().split(',')))
    print(sum(lanternfish(t, 80) for t in fish_input))
    print(sum(lanternfish(t, 256) for t in fish_input))

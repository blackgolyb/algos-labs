import matplotlib.pyplot as plt
import numpy as np
import numba


@numba.njit()
def error(A, B):
    return ((A - B) ** 2).mean()


@numba.njit()
def hash_func(value, base, size):
    res = 0
    i = 0

    while value > 0:
        res += (value % 10) * (base**i)
        value //= 10
        i += 1

    return res % size

    # return np.random.randint(0, size+1, 1)[0]
    # return value % size


@numba.njit(parallel=True)
def test(bases, sizes, numbers):
    result = np.empty((bases.size, sizes.size), np.float64)
    n = numbers.size

    for i in numba.prange(sizes.size):
        size = sizes[i]
        exp = n / size
        test = np.zeros(size)

        for j in numba.prange(bases.size):
            base = bases[j]
            test.fill(0)
            for num in numbers:
                test[hash_func(num, base, size)] += 1

            result[j][i] = error(test, exp)

    return result


def is_prime(n):
    if n % 2 == 0 and n > 2:
        return False
    return all(n % i for i in range(3, int(n**0.5) + 1, 2))


def primes_grid(x, y):
    max_num = max(np.max(x), np.max(y))
    numbers = np.arange(0, max_num + 1)
    is_prime_v = np.vectorize(is_prime)
    prime = is_prime_v(numbers)
    res = list()

    for i in x:
        for j in y:
            if prime[i] and prime[j]:
                res.append((i, j))

    return np.array(res)


def pows_2_grid(x, y):
    max_num = max(np.max(x), np.max(y))
    res = list()
    pows = set()

    current = 1
    while current <= max_num:
        pows.add(current)
        current *= 2

    for i in x:
        for j in y:
            if i in pows and j in pows:
                res.append((i, j))

    return np.array(res)


def main():
    n = 1050
    # n = 100
    bases = np.arange(2, 257)
    sizes = np.arange(8, n)
    np.random.seed(seed=2134)
    numbers = np.random.randint(0, 18_446_744_073_709_551_615 // 2 - 1, n * 5)

    # with open("test.npy", "rb") as f:
    #     res = np.load(f)

    res = test(bases, sizes, numbers)

    with open("test.npy", "wb") as f:
        np.save(f, res)

    # res = np.log10(res)

    X, Y = np.meshgrid(sizes, bases)
    # ax = plt.axes(projection='3d')
    # ax.plot_surface(X, Y, res, rstride=1, cstride=1, cmap='viridis', edgecolor='none', antialiased=False)

    primes = primes_grid(sizes, bases)
    pows_2 = pows_2_grid(sizes, bases)

    plt.contourf(
        X,
        Y,
        res,
        levels=100,
        origin="lower",
        cmap="viridis",
    )
    # plt.scatter(primes[:, 0], primes[:, 1], c='w', linewidths=1, alpha=0.3)
    # plt.scatter(pows_2[:, 0], pows_2[:, 1], c='r', linewidths=1, alpha=0.5)

    plt.show()


if __name__ == "__main__":
    main()

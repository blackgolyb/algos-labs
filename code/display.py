import csv
import matplotlib.pyplot as plt

def lab2():
    n = []
    ie = []
    re = []
    i = []
    r = []

    with open("perf_lab2.csv", newline="") as csvfile:
        reader = csv.DictReader(csvfile)
        for row in reader:
            n.append(int(row["n"]))
            ie.append(int(row["iterational_effective"]))
            re.append(int(row["recursive_effective"]))
            i.append(int(row["iterational"]))
            # r.append(int(row["recursive"]))

    plot = lambda y, name: plt.plot(n, y, label=name)

    plot(ie, "iterational effective")
    plot(re, "recursive effective")
    plot(i, "iterational")
    # plot(r, "recursive")

    plt.title("lab2")
    plt.xlabel("n")
    plt.ylabel("Time")
    plt.legend()
    plt.show()


def lab9():
    n = []
    lc = []
    bc = []
    ls = []
    bs = []
    lt = []
    bt = []

    with open("perf_lab9.csv", newline="") as csvfile:
        reader = csv.DictReader(csvfile)
        for row in reader:
            n.append(int(row["n"]))
            lc.append(int(row["linear_compares"]))
            bc.append(int(row["binary_compares"]))
            ls.append(int(row["linear_shifts"]))
            bs.append(int(row["binary_shifts"]))
            lt.append(int(row["linear_time"]))
            bt.append(int(row["binary_time"]))

    plot = lambda y, name: plt.plot(n, y, label=name)

    plot(lc, "Лінійний пошук")
    plot(bc, "Бінарний пошук")
    plt.title("Кількість порівнянь")
    plt.xlabel("Кількість елементів")
    plt.ylabel("Кількість порівнянь")
    plt.legend()
    plt.show()

    # plot(ls, "Лінійний пошук")
    # plot(bs, "Бінарний пошук")
    # plt.title("Shifts")
    # plt.xlabel("n")
    # plt.ylabel("Shifts")
    # plt.legend()
    # plt.show()

    plot(lt, "Лінійний пошук")
    plot(bt, "Бінарний пошук")
    plt.title("Час пошуку (наносекунди) ")
    plt.xlabel("Кількість елементів")
    plt.ylabel("Час пошуку (наносекунди) ")
    plt.legend()
    plt.show()


def lab10():
    n = []
    lc = []
    bc = []
    hc = []
    lt = []
    bt = []
    ht = []

    with open("perf_lab10.csv", newline="") as csvfile:
        reader = csv.DictReader(csvfile)
        for row in reader:
            n.append(int(row["n"]))
            lc.append(int(row["linear_compares"]))
            bc.append(int(row["binary_compares"]))
            hc.append(int(row["hash_compares"]))
            lt.append(int(row["linear_time"]))
            bt.append(int(row["binary_time"]))
            ht.append(int(row["hash_time"]))

    plot = lambda y, name: plt.plot(n, y, label=name)

    plot(lc, "Лінійний пошук")
    plot(bc, "Бінарний пошук")
    plot(hc, "Хеш-таблиця")
    plt.title("Кількість порівнянь")
    plt.xlabel("Кількість елементів")
    plt.ylabel("Кількість порівнянь")
    plt.legend()
    plt.show()

    plot(lt, "Лінійний пошук")
    plot(bt, "Бінарний пошук")
    plot(ht, "Хеш-таблиця")
    plt.title("Час пошуку (наносекунди) ")
    plt.xlabel("Кількість елементів")
    plt.ylabel("Час пошуку (наносекунди) ")
    plt.legend()
    plt.show()


def sort_plot(*sorts):
    data = {}
    for name in sorts:
        data[name] = {
            "n": [],
            "c": [],
            "s": [],
            "t": [],
        }

        with open(f"{name}.csv", newline="") as csvfile:
            reader = csv.DictReader(csvfile)
            for row in reader:
                data[name]["n"].append(int(row["n"]))
                data[name]["c"].append(int(row["compares"]))
                data[name]["s"].append(int(row["swaps"]))
                data[name]["t"].append(int(row["time"]))

    for name, d in data.items():
        plt.plot(d["n"], d["c"], label=name)
    plt.title("Кількість порівнянь")
    plt.xlabel("Кількість елементів")
    plt.ylabel("Кількість порівнянь")
    plt.legend()
    plt.show()

    for name, d in data.items():
        plt.plot(d["n"], d["s"], label=name)
    plt.title("Кількість пересилань")
    plt.xlabel("Кількість елементів")
    plt.ylabel("Кількість пересилань")
    plt.legend()
    plt.show()

    for name, d in data.items():
        plt.plot(d["n"], d["s"], label=name)
    plt.title("Час виконання")
    plt.xlabel("Кількість елементів")
    plt.ylabel("Час виконання")
    plt.legend()
    plt.show()


def main():
    # lab2()
    # lab9()
    # lab10()
    # sort_plot("bubble")
    # sort_plot("heap", "radix", "bucket")
    sort_plot("turnament")


if __name__ == "__main__":
    main()

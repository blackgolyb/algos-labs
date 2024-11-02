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


def main():
    # lab2()
    lab9()

if __name__ == "__main__":
    main()

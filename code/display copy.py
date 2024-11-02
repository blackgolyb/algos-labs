import csv
import matplotlib.pyplot as plt

def lab9():
    n = []
    lc = []
    bc = []
    lt = []
    bt = []

    with open("perf_lab9.csv", newline="") as csvfile:
        reader = csv.DictReader(csvfile)
        for row in reader:
            n.append(int(row["n"]))
            lc.append(int(row["linear_compares"]))
            bc.append(int(row["binary_compares"]))
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

    plot(lt, "Лінійний пошук")
    plot(bt, "Бінарний пошук")
    plt.title("Час пошуку (наносекунди) ")
    plt.xlabel("Кількість елементів")
    plt.ylabel("Час пошуку (наносекунди) ")
    plt.legend()
    plt.show()


def main():
    lab9()

if __name__ == "__main__":
    main()

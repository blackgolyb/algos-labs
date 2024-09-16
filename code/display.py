import csv
import matplotlib.pyplot as plt

def lab2():
    n = []  
    e = []
    i = []
    r = []

    with open("perf_lab2.csv", newline="") as csvfile:
        reader = csv.DictReader(csvfile)
        for row in reader:
            n.append(int(row["n"]))
            e.append(int(row["effective"]))
            i.append(int(row["iterational"]))
            r.append(int(row["recursive"]))

    plot = lambda y, name: plt.plot(n, y, label=name)

    plot(e, "effective")
    plot(i, "iterational")
    plot(r, "recursive")
    
    plt.show()

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

    plot(lc, "linear")
    plot(bc, "binary")
    plt.show()
    
    plot(lt, "linear")
    plot(bt, "binary")
    plt.show()


def main():
    lab9()

if __name__ == "__main__":
    main()

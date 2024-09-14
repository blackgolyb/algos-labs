import csv
import matplotlib.pyplot as plt

def main():
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

if __name__ == "__main__":
    main()

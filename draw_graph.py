import matplotlib.pyplot as plt


def read_file(vertex_file, edge_file):
    v_list = list()
    e_list = list()
    with open(vertex_file) as vfobj:
        for line in vfobj:
            v_list.append([float(x) for x in line.strip().split(',')])

    with open(edge_file) as efobj:
        for line in efobj:
            e_list.append([int(x) for x in line.strip().split(' ')])

    return (v_list, e_list[0])


if __name__ == "__main__":
    vertex_file = 'randGraph.txt'
    edge_file = 'edge.txt'
    figure_file = 'convexhull.png'

    (vlist, elist) = read_file(vertex_file, edge_file)

    vlistx = [x[0] for x in vlist]
    vlisty = [x[1] for x in vlist]

    elistx = list()
    elisty = list()
    elistx = [vlist[x-1][0] for x in elist]
    elisty = [vlist[x-1][1] for x in elist]

    plt.scatter(vlistx, vlisty)
    plt.plot(elistx, elisty, 'r')
    plt.savefig(figure_file)
    plt.show()

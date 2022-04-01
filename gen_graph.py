import random
import argparse

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('-l', "--length", type=float,
                        default=100.0, help="set length")
    parser.add_argument('-w', "--width", type=float,
                        default=100.0, help="set width")
    parser.add_argument('-n', "--number", type=int,
                        default=100, help="set point number")
    parser.add_argument('-f', "--filename",
                        default='randGraph.txt', help="the saved file name")
    args = parser.parse_args()

    length = args.length
    width = args.width
    point_num = args.number
    filename = args.filename

    print("Generate {} points in length {}, width {}".format(
        point_num, length, width))

    with open(filename, 'w') as fobj:
        for i in range(point_num):
            fobj.write(str(random.uniform(-1*length/2, length/2)) +
                       ','+str(random.uniform(-1*width/2, width/2))+'\n')
        print("Successfully save to {}".format(filename))

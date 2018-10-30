from random import randint
from time import time

import numpy as np

from merkle_tree import MerkleTree
from merkle_avl_tree import MerkleAvlTree


def test():
    # tree = MerkleTree()
    tree = MerkleAvlTree()

    lst = [randint(1, 100000000000) for _ in range(10)]
    for i in lst:
        tree.add(i, i)

    print(tree)

    print("Depth =", tree.root.depth)
    print("Hash =", tree.root.hash)


def statistics(klass):
    count = 10000
    rng = 100000000000

    t_delta_lst = []
    depth_lst = []
    for _ in range(100):
        tree = klass()

        t_begin = time()
        for _ in range(count):
            i = randint(1, rng)
            tree.add(i, i)
        t_end = time()

        t_delta = t_end - t_begin
        depth = tree.root.depth

        t_delta_lst.append(t_delta)
        depth_lst.append(depth)

    print(klass.__name__)

    print("Average time:", np.mean(t_delta_lst))
    print("Median time:", np.median(t_delta_lst))
    print("Minimal time:", np.min(t_delta_lst))
    print("Maximal time:", np.max(t_delta_lst))

    print("Average depth:", np.mean(depth_lst))
    print("Median depth:", np.median(depth_lst))
    print("Minimal depth:", np.min(depth_lst))
    print("Maximal depth:", np.max(depth_lst))

    print()


if __name__ == "__main__":
    # test()

    statistics(MerkleTree)
    statistics(MerkleAvlTree)

# AVL Merkle Tree

## Abstract

...


## What is Merkle tree?

Merlke tree is a special structure of hashes. Generally, it is a binary tree over a data set in which each node keeps a hash and only the last (lowest) nodes refer to the data elements (represented as leaves). Each leaf has a hash of the bound element, and the hash of a node is a hash of hashes of child nodes (or leaves) unified with a function. In other words, Merkle tree is an auxiliary structure of hashes over a data set, created for some purposes that we will consider carefully below.

Look at the picture. Here we see how an array of integers can be represented with the help of an ordinary binary tree and with a Merkle tree. The main difference is the binary tree keeps an element in each node, but the Merkle tree keeps them only in the bottom nodes, the other nodes are for hashes only.

(a picture of a binary tree and a merkle tree for a set of integer numbers)

What is the goals of creating such structure as Merlke tree? As you can notice, a node's hash depends on the hashes of all its subtrees, particularly, the root node's hash is the result of calculations for all data elements, so it is called as a hash of tree or a root hash. Also there is a hash path to each element that can be used to show (or to prove) existing of the element providing its hash path and adjacent hashes for each node in the path as shown on the picture.

(a picture of hash path)

To understand it better, we should consider a simple and classical example. Supposing we have a data set with the elements we do not want to reveal. First, we build a tree of hashes over the elements as described above. And we make the root hash public. Supposing, somebody wants to prove that an element is in the data set. All he has to do is to provide the hash path (with all	adjacent hashes) that leads from the element to the root hash that is known because it is public. This hash path is impossible to fake (it is equal to a hash reversing task), so this path may be a sufficient proof itself. Here we suppose the hash function is public and the hash of target element can be easily calculated with the function. For the case above the proof would be like this:

...(path of adjacent hashes to the root)...

And the check is following:

...(applying hash function to the given hashes and getting the root path in the end)...

Thus, Merkle trees can be used to hide data sets but having a way to provide all the necessary proofs. Also this approach is a good way to sign some data to verify it easier, for example, it is widely used in blockchain to get a hash of a block. Additionally Merkle trees have advantages regarding the number of calculations that should be done after adding or removing an element, because we need to modify the path nodes' hashes only. Some more information you can find in wikipedia: https://en.wikipedia.org/wiki/Merkle_tree


## The problem

Above we describe what the Merkle tree is. It looks quite conveniet for some cryptography tasks, it is helpful in Bitcoin, Ethereum. But there are some cases where the standard way to build a Merkle tree is not enough. Mostly this is this article is written for. For example, such tasks may be too sensitive to the size of proof. In other words, if the result tree is too deep the proving path of hashes has too many hashes that make a problem to store in memory or to calculate during a proper time. So we can require a balanced tree of hashes instead, because it ensures the small logarithmic depth even in the worst case. Below we compare numerically two ways to build Merkle trees.


## AVL tree, how it works

AVL tree is a binary tree that changes its structure after adding or removing an element, in order to keep the total depth (the maximal distance between the root and a node) as less as possible. On the picture there are examples of balanced and unbalanced trees.

(a picture of balanced and unbalanced trees)

In AVL tree the balance is reached by too types of rotations in the nodes. For example, if an element is added (or removed), we check each node in the path to the element for the difference in depths of its left and right subtrees. If the difference is 2 we make a rotation according to the rules invented and discovered by Adelson-Velsky and Landis (this is why such tree is called AVL). There four sort of rotations:

(images of AVL rotations)

After the correct rotation for the node, its depths difference will be 0 or 1, thus the tree will be balanced.

Comparing to the ordinary binary tree that has the complexity O(N) in worst case, the AVL tree tree has O(log N), but rebalancing takes consumes some performance obviously. More about AVL tree is in wikipedia: https://en.wikipedia.org/wiki/AVL_tree

Except for AVL tree, there are many different ways to achieve the balance in binary trees. Here are some of them:

* Red-black tree
* Splay tree
* Scapegoat tree
* B-tree
* T-tree
* etc...

We chose AVL tree as a way that is quite simple and comvenient enough to show how the Merkle tree can be made balanced. AVL tree is not the only possible approach and, of course, there are many different ways to get the same as well.


## Prototype of AVL Merkle tree with Python

To discover the AVL approach regarding Merkle trees we combine them together and implement all of this in Python.

We store elements in leaves, so we create this class for it.

```python
class Leaf:
    def __init__(self, key, data):
        self.key = key
        self.data = data

    def __repr__(self):
        return "Leaf(key={},data={})".format(self.key, self.data)
```

As mentioned above, we need to keep tree nodes for hashes:

```python
class Node:
    def __init__(self, key, parent=None, leaf=None):
        self.key = key
        self.parent = parent
        self.leaf = leaf
        self.children = [None, None]
        self.depth = 1
        self.hash = None
        self._update_hash()

    def __repr__(self):
        return "Node(key={},depth={},hash='{}')".format(self.key, self.depth, self.hash)

    def bind(self, parent, index):
        """
        Binds the node to the given parent.
        If index = 0 the node will the added on the left.
        If index = 0 the node will the added on the left.
        It's supposed the node has not parent yet.
        """
        parent.children[index] = self
        self.parent = parent

    def unbind(self):
        """
        Upbinds the node from its parent.
        It's supposed the node has parent.
        """
        parent = self.parent
        index = parent.children.index(self)
        parent.children[index] = None
        self.parent = None
        return parent, index

    def update(self):
        """
        Updates depth and hash
        """
        self._update_depth()
        self._update_hash()

    def _update_depth(self):
        self.depth = 1 + max(child.depth for child in self.children)

    def _update_hash(self):
        if self.leaf is None:
            sum_hash = ''.join(child.hash for child in self.children)
            self.hash = merkle_hash(sum_hash)
        else:
            self.hash = merkle_hash(str(self.key))
```

As you can see, we keep depth in the node. It is done to make calculations easier. Also there are to additional methods "bind" and "unbind" that help us manage the AVL rotations and other work with the tree structure.

The code for Merkle tree looks like this:

```python
class MerkleTree:
    def __init__(self):
        self.root = None

    def __repr__(self):
        """
        Prints the tree prettily.
        """
        if self.root is not None:
            return "\n".join(self._print_node(self.root, ""))
        else:
            return "None"

    def get(self, key):
        """
        Gets data by key.
        """
        node = self._find_node(key)
        if node.key == key:
            return node.leaf.data
        else:
            raise KeyError(key)

    def add(self, key, data):
        """
        Adds a new element given by key and data.
        """
        leaf = Leaf(key, data)
        if self.root is None:
            self.root = Node(key, leaf=leaf)
        else:
            node = self._find_node(key)
            if node.key == key:
                raise KeyError(key)
            self._add_leaf_to_node(leaf, node)

    def _find_node(self, key):
        node = self.root
        while node.leaf is None:
            index = int(key > node.key)
            node = node.children[index]
        return node

    def _print_node(self, node, prefix=""):
        if node.leaf is not None:
            yield "{}{} -> {}".format(prefix, node, node.leaf)
        else:
            yield "{}{}".format(prefix, node)
            for n in node.children:
                yield from self._print_node(n, prefix + "   ")

    def _add_leaf_to_node(self, leaf, node):
        old_leaf_node = Node(key=node.key, leaf=node.leaf)
        new_leaf_node = Node(key=leaf.key, leaf=leaf)
        index = int(node.key > leaf.key)
        old_leaf_node.bind(node, index)
        new_leaf_node.bind(node, 1 - index)
        if index:
            node.key = leaf.key
        node.leaf = None
        self._update_node(node, recursive=True)

    def _update_node(self, node, recursive=False):
        while node is not None:
            node.update()
            if not recursive:
                break
            node = node.parent
```

Here we did not implement the method "remove", we suggest the reader to do it by himself as an exercise. Also we suppose the keys are comparable (the operator > is defined for them), so we are able to sort the elements by keys. The children in a node are sorted, the key of the first child (left node) is less than of the second one (right node). What makes this tree Merkle is the method "_update_node" that is called after adding a new element, it recalculates all the hashes in the path to the added node from the root.

Well, to make the Merkle tree balanced all we need to do is to inherit from the implemented tree and add the necessary rotations after adding a node:

```python
class MerkleAvlTree(MerkleTree):
    def _add_leaf_to_node(self, leaf, node):
        super()._add_leaf_to_node(leaf, node)
        disbalanced_node = self._search_for_disbalance_from_node(node)
        if disbalanced_node is not None:
            self._rebalance_node(disbalanced_node)

    @classmethod
    def _search_for_disbalance_from_node(cls, node):
        while node is not None:
            if abs(node.children[0].depth - node.children[1].depth) >= 2:
                return node
            node = node.parent
        return None

    def _rebalance_node(self, node):
        if node.children[0].depth > node.children[1].depth:
            if node.children[0].children[0].depth > node.children[0].children[1].depth:
                self._simple_rotate(node, node.children[0], node.children[0].children[1], index=0)
            else:
                self._double_rotate(node, node.children[0], node.children[0].children[1],
                                    node.children[0].children[1].children[0], node.children[0].children[1].children[1],
                                    index=0)
        else:
            if node.children[1].children[1].depth > node.children[1].children[0].depth:
                self._simple_rotate(node, node.children[1], node.children[1].children[0], index=1)
            else:
                self._double_rotate(node, node.children[1], node.children[1].children[0],
                                    node.children[1].children[0].children[1], node.children[1].children[0].children[0],
                                    index=1)

    def _simple_rotate(self, top_node, side_node, mid_node, index):
        side_node.unbind()
        mid_node.unbind()

        if top_node is not self.root:
            top_parent, top_index = top_node.unbind()
            side_node.bind(top_parent, top_index)
        else:
            self.root = side_node

        top_node.bind(side_node, 1 - index)
        mid_node.bind(top_node, index)

        self._update_node(top_node, recursive=True)

    def _double_rotate(self, top_node, side_node, mid_node, mid1_node, mid2_node, index):
        side_node.unbind()
        mid_node.unbind()
        mid1_node.unbind()
        mid2_node.unbind()

        if top_node is not self.root:
            top_parent, top_index = top_node.unbind()
            mid_node.bind(top_parent, top_index)
        else:
            self.root = mid_node

        side_node.bind(mid_node, index)
        top_node.bind(mid_node, 1 - index)
        mid1_node.bind(side_node, 1 - index)
        mid2_node.bind(top_node, index)

        self._update_node(top_node, recursive=False)
        self._update_node(side_node, recursive=False)
        self._update_node(mid_node, recursive=True)
```

The rotations are implemented as it would be made in the standard AVL tree. In our case of Merkle tree we have to update all the necessary hashes carefully, that is important.


## Numerical comparison

To compare the performances of two approaches we wrote such script:

```python
from time import time
from uuid import uuid4

import numpy as np

from merkle_tree import MerkleTree
from merkle_avl_tree import MerkleAvlTree


def statistics(klass):
    count = 10000

    t_delta_lst = []
    depth_lst = []
    for _ in range(100):
        tree = klass()

        t_begin = time()
        for _ in range(count):
            i = uuid4()
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
    statistics(MerkleTree)
    statistics(MerkleAvlTree)
```

Here we fill a tree with 10000 elements (that are random integers from the range from 1 to 100000000000) 100 times and measure time and depth statistics. And on our computer (CPU: Intel(R) Core(TM) i7-4700HQ CPU @ 2.40GHz) we get following results:

```
MerkleTree
Average time: 0.610906083584
Median time: 0.595646500587
Minimal time: 0.544233560562
Maximal time: 0.854603052139
Average depth: 32.04
Median depth: 32.0
Minimal depth: 29
Maximal depth: 38

MerkleAvlTree
Average time: 0.698779945374
Median time: 0.698184490204
Minimal time: 0.641226768494
Maximal time: 0.771322011948
Average depth: 17.0
Median depth: 17.0
Minimal depth: 17
Maximal depth: 17
```

As we can see, generally, regarding the time unbalanced tree is around 15% faster, but the building time for the balanced tree has less variance and can be more predictable (maximal time is better to the balanced tree). But with respect to the depth, the difference is significant: 17 against 32, that is quite huge considering the not big size of the tree (10000 elements).

We also tested the trees with bigger sizes. That is what we get:

|elements |unbalanced|balanced|
|--------:|---------:|-------:|
|1        |1         |1       |
|10       |7         |5       |
|100      |14        |9       |
|1000     |23        |13      |
|10000    |31        |17      |
|100000   |42        |21      |
|1000000  |52        |25      |

Averagely, the unbalanced tree is two times deeper than the balanced one.


## Conclusion

...

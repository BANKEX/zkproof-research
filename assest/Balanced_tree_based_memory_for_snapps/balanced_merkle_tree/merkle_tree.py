import hashlib


def merkle_hash(s):
    return hashlib.sha256(s.encode()).hexdigest()


class MerkleTree:
    class Leaf:
        def __init__(self, key, data):
            self.key = key
            self.data = data

        def __repr__(self):
            return "Leaf(key={},data={})".format(self.key, self.data)

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
            assert self.parent is None
            assert parent.children[index] is None
            parent.children[index] = self
            self.parent = parent

        def unbind(self):
            assert self.parent is not None
            parent = self.parent
            index = parent.children.index(self)
            parent.children[index] = None
            self.parent = None
            return parent, index

        def update(self):
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

    def __init__(self):
        self.root = None

    def print(self):
        if self.root is not None:
            self._print_node(self.root, "")
        else:
            print(None)

    def find_node(self, key):
        node = self.root
        while node.leaf is None:
            index = int(key > node.key)
            node = node.children[index]
        return node

    def add_leaf(self, key, data):
        leaf = self.Leaf(key, data)
        if self.root is None:
            self.root = self.Node(key, leaf=leaf)
        else:
            node = self.find_node(key)
            self._add_leaf_to_node(leaf, node)

    def _print_node(self, node, prefix=""):
        if node.leaf is not None:
            print("{}{} -> {}".format(prefix, node, node.leaf))
        else:
            print("{}{}".format(prefix, node))
            for n in node.children:
                self._print_node(n, prefix + "   ")

    def _add_leaf_to_node(self, leaf, node):
        old_leaf_node = self.Node(key=node.key, leaf=node.leaf)
        new_leaf_node = self.Node(key=leaf.key, leaf=leaf)
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

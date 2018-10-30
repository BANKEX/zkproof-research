import hashlib


def merkle_hash(s):
    return hashlib.sha256(s.encode()).hexdigest()


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
        Updates depth and hash.
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

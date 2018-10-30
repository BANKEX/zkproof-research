from merkle_tree import MerkleTree


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

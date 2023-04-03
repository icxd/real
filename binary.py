from dataclasses import dataclass

@dataclass
class Node:
    value: int
    left: "Node"
    right: "Node"

def invert(node: Node, indent: int = 0) -> Node:
    print(f'{"  "*indent}- {node.value}')
    if node.left and node.right:
        node.left, node.right = node.right, node.left
        invert(node.left, indent + 1)
        invert(node.right, indent + 1)

def main():
    node = Node(1, Node(2, Node(4, None, None), Node(5, None, None)), Node(3, Node(6, None, None), Node(7, None, None)))

    invert(node)

main()
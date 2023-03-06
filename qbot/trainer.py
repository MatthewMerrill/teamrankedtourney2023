import numpy as np

import newcular
from pprint import pprint

from net import Model

if __name__ == '__main__':
    print('hello!')
    # board = newcular.load_board(tuple())
    # pprint(vars(board))

    # Model().save('test_model')

# class PotentialAction:
#     def __init__(self, prv, act, nxt, n, w, q, p):
#         """
#         :param n: the number of times action act has been taken from state prv
#         :param w: the total value of nxt
#         :param q: the mean value of nxt
#         :param p: the prior probability of selecting act
#         """
#         self.n = n
#         self.w = w
#         self.q = q
#         self.p = p
#
# class Node:
#     def __init__(self, board, prior):
#         self.board = board
#         self.prior = prior
#         self.move_probs = {}
#
#     # def load_actions(self, move):
#     #     new_history = self.board.history + [move]
#     #     nxt = newcular.load_board(new_history)
#     #     model.pred
#     #     return PotentialAction(self, move, nxt, 0, 0, 0, )
#     # def select(self):
#     #     for self.board.valid_moves;
#
#     def set_policy(self, action_probs):
#         self.move_probs = {}
#         for move in self.board.valid_moves:
#             from_row, from_col, dest_row, dest_col = interpret_move_as_axes(move)
#             self.move_probs[move] = action_probs[from_row][from_col][dest_row][dest_col]
#         return self.move_probs
#
# def interpret_move_as_axes(move):
#     from_col_str, from_row_str, dest_col_str, dest_row_str = tuple(move)
#     from_row = int(from_row_str)
#     from_col = ord(from_col_str) - ord('A')
#     dest_row = int(dest_row_str)
#     dest_col = ord(dest_col_str) - ord('A')
#     return from_row, from_col, dest_row, dest_col
# def interpret_moves_as_mask(moves):
#     mask = np.zeros((9, 7, 9, 7))
#     for move in moves:
#         from_row, from_col, dest_row, dest_col = interpret_move_as_axes(move)
#         mask[from_row][from_col][dest_row][dest_col] = 1
#     return mask
#
# class Trainer:
#     def __init__(self, model):
#         self.model = model
#
#     def record(self, node, pred_value, pred_policy, real_value):
#
#
#     def run(self):
#
#         root = Node(newcular.load_board(tuple()))
#         pred_value, pred_policy = self.model.predict(root.board.representation)
#         action_probs = pred_policy * interpret_moves_as_mask(root.board.valid_moves)
#         action_probs /= np.sum(action_probs)
#         root.set_policy(action_probs)
#
#         for _ in range(400):
#             node = root
#             search_path = [node]
#             return node.board.winner
#
#         pred_value, pred_policy = self.model.predict([node.board.representation])
#         self.record(node, pred_value, pred_policy, real_value)
#
#
#     def mcts_evaluate(self, node):
#         for move in node.board.valid_moves:

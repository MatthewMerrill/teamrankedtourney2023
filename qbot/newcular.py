
from collections import namedtuple
from typing import Tuple, List

import requests
import functools

class Move:
    def __init__(self, from_rc: Tuple[int, int], dest_rc: Tuple[int, int]):
        self.from_rc = from_rc
        self.dest_rc = dest_rc

    def invert(self):
        return Move(
            (9 - self.from_rc[0], self.from_rc[1]),
            (9 - self.dest_rc[0], self.dest_rc[1])
        )

    def get_from_dest(self):
        return (self.from_rc, self.dest_rc)

class Board:
    def __init__(self, valid_moves, render, winner, representation):
        self.valid_moves = valid_moves
        self.render = render
        self.winner = winner
        self.representation = representation

@functools.lru_cache(maxsize=10000)
def load_board(moves: List[str]):
    res = requests.get('http://localhost:8181/gameType/newcular/summary/' + ' '.join(moves))
    res_json = res.json()
    return Board(
        res_json['valid_moves'],
        res_json['render'],
        res_json['winner'],
        res_json['representation'])

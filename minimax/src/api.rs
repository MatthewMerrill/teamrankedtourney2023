use newcular::board::Board;

trait Evaluator<B> where B: Board {
    fn mini(&self, board: &SimpleBoard, plies: u8) -> EvalResult;
    fn maxi(&self, board: &SimpleBoard, plies: u8) -> EvalResult;
}
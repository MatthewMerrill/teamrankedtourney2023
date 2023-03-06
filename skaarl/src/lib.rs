use tensorflow::{
    Graph, Operation, SavedModelBundle, Session, SessionOptions, SessionRunArgs, Tensor,
};

struct Evaluator {
    bundle: SavedModelBundle,
    input_op: Operation,
    value_head_op: Operation,
    policy_head_op: Operation,
}

impl Evaluator {
    fn new(save_dir: &str) -> Evaluator {
        // In this file test_in_input is being used while in the python script,
        // that generates the saved model from Keras model it has a name "test_in".
        // For multiple inputs _input is not being appended to signature input parameter name.
        let signature_input_parameter_name = "input";
        let signature_value_parameter_name = "value_head";
        let signature_policy_parameter_name = "policy_head";

        let mut graph = Graph::new();

        // Load saved model bundle (session state + meta_graph data)
        let bundle =
            SavedModelBundle::load(&SessionOptions::new(), &["serve"], &mut graph, save_dir)
                .expect("Can't load saved model");

        // Get signature metadata from the model bundle
        let signature = bundle
            .meta_graph_def()
            .get_signature("serving_default")
            .unwrap();

        // Get input/output info
        let input_info = signature.get_input(signature_input_parameter_name).unwrap();
        let value_output_info = signature
            .get_output(signature_value_parameter_name)
            .unwrap();
        let policy_output_info = signature
            .get_output(signature_policy_parameter_name)
            .unwrap();

        // Get input/output ops from graph
        let input_op = graph
            .operation_by_name_required(&input_info.name().name)
            .unwrap();
        let value_head_op = graph
            .operation_by_name_required(&value_output_info.name().name)
            .unwrap();
        let policy_head_op = graph
            .operation_by_name_required(&policy_output_info.name().name)
            .unwrap();

        Evaluator {
            bundle,
            input_op,
            value_head_op,
            policy_head_op,
        }
    }

    fn evaluate(&self, values: &[f32; 441]) {
        // Get the session from the loaded model bundle
        let session = &self.bundle.session;

        // Initialize save_dir, input tensor, and an empty graph
        let input_tensor: Tensor<f32> = Tensor::new(&[1, 7, 7, 9])
            .with_values(values)
            .expect("Can't create tensor");

        // Manages inputs and outputs for the execution of the graph
        let mut args = SessionRunArgs::new();
        args.add_feed(&self.input_op, 0, &input_tensor); // Add any inputs

        let out = args.request_fetch(&self.value_head_op, 0); // Request outputs

        // Run model
        session
            .run(&mut args) // Pass to session to run
            .expect("Error occurred during calculations");

        // Fetch outputs after graph execution
        let out_res: f32 = args.fetch(out).unwrap()[0];

        println!("Results: {:?}", out_res);
    }
}

mod test {
    use crate::Evaluator;

    #[test]
    fn test_eval() {
        let eval = Evaluator::new("../qbot/test_model");
        eval.evaluate(&[0f32; 441]);
    }
}

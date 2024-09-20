import { useEffect, useRef, useState } from 'react'
// import reactLogo from './assets/react.svg'
// import viteLogo from '/vite.svg'
import './App.css'

import EquationInput from './EquationInput'
import Graph from './Graph';
import Intercepts from './Intercepts'

import init, { Evaluator, setup } from "./wasm-graph-calc/pkg/wasm_graph_calc.js"
import { evaluator_get_graph_names } from './wasm-graph-calc/pkg/wasm_graph_calc_bg.wasm.js';

function App() {

	const [wasmLoaded, setWasmLoaded] = useState(false);
	
	useEffect(() => {

		const wasmSetup = async () => {
			await init();
			setup(); // set panic hook
			setWasmLoaded(true);
		}

		wasmSetup()
	});

	const [equations, setEquations] = useState([
		"",
	]);

	const [answers, setAnswers] = useState<any[]>([
		null
	]);

	const [graphs, setGraphs] = useState<any[]>([]);

	const [intercepts, setIntercepts] = useState<number[][]>([]);

	const [steps, setSteps] = useState(0);
	const [epsilon, setEpsilon] = useState(0);

	const [eq1, setEq1] = useState("");
	const [eq2, setEq2] = useState("");

	const [graphNames, setGraphNames] = useState<String[]>([])

	// update answers and graphs when equations change
	useEffect(() => {

		if (!wasmLoaded) {
			setAnswers(
				equations.map(() => {
					return undefined;
				})
			);

			return;
		}

		let evaluator = new Evaluator();
		let data = [...equations];

		let new_answers: any[] = [];
		let new_graphs: any[] = [];

		data.forEach(eq => {
			let e = evaluator.evaluate(eq);

			if (!e) {
				new_answers.push(undefined);
			} else if (e.type == "Graph") {
				new_graphs.push(e.points);
				new_answers.push(undefined);
			} else {
				new_answers.push({value: e.value, var_name: e.var_name});
			}
		});

		let raw_intercept_list = evaluator.find_intercepts(eq1, eq2, steps, epsilon);

		if (raw_intercept_list && raw_intercept_list.length > 0) {
			let raw_intercept_list2 = [...raw_intercept_list];
			let intercept_list: number[][] = [];

			while (raw_intercept_list2.length > 0) {

				intercept_list.push([raw_intercept_list2[0], raw_intercept_list2[1]]);
				raw_intercept_list2.splice(0, 2);
			}

			setIntercepts(intercept_list);

		} else {
			console.log("there are no intercepts")
			setIntercepts([])
		}

		setAnswers(new_answers);
		setGraphs(new_graphs);

		let graph_name_list: String[] = [...evaluator.get_graph_names()].map((x) => String.fromCharCode(x));
		setGraphNames(graph_name_list);

		// to avoid memory leaks as wasm does not automatically free structs
		evaluator.free();

	}, [equations, eq1, eq2, wasmLoaded, steps, epsilon]);

  return (
	<>

		<div className="topBar">
			<div><button>Save</button></div>
			<div><button>Load</button></div>
		</div>

		<div className="middleContent">
			<div className="split left">
				<EquationInput 
					equations={equations}
					setEquations={setEquations}

					answers={answers}
				/>
			</div>
			<div className="split right">
				<Graph

					graphs={graphs}
					intercepts={intercepts}
				/>
			</div>
			<div className="intercepts">
				<Intercepts 
					eq1={eq1}
					setEq1={setEq1}

					eq2={eq2}
					setEq2={setEq2}

					steps={steps}
					setSteps={setSteps}

					epsilon={epsilon}
					setEpsilon={setEpsilon}

					intercepts={intercepts}
				/>
			</div>
		</div>
    </>
  )
}

export default App;
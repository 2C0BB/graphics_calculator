import { useEffect, useRef, useState } from 'react'
// import reactLogo from './assets/react.svg'
// import viteLogo from '/vite.svg'
import './App.css'

import EquationInput from './EquationInput'
import Graph from './Graph';

import init, { Evaluator, EvaluatorResponse, setup } from "./wasm-graph-calc/pkg/wasm_graph_calc.js"

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

	const [answers, setAnswers] = useState<(undefined | number)[]>([
		undefined
	]);

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

		setAnswers(data.map(eq => {
			return evaluator.evaluate(eq);
		}));

		evaluator.free();

		//console.log(answers);

	}, [equations, wasmLoaded]);

  return (
	<>
		<div className="split left">
			<EquationInput 
				equations={equations}
				setEquations={setEquations}

				answers={answers}

				wasmLoaded={wasmLoaded}
			/>
		</div>
		<div className="split right">
			<Graph
				equations={equations}
				setEquations={setEquations}

				wasmLoaded={wasmLoaded}
			/>
		</div>
    </>
  )
}

export default App;
import { useEffect, useState } from "react";
import { Evaluator, EvaluateResponse, EvaluatorResponse } from "./wasm-graph-calc/pkg/wasm_graph_calc.js";

function EquationInput({
		equations,
		setEquations,

		wasmLoaded
	}: {
		equations: any, 
		setEquations: any,

		wasmLoaded: boolean
	}) {

	function handleEquationChange(
		idx: number,
		event: React.ChangeEvent<HTMLInputElement>
	) {

		let data = [...equations];
		data[idx] = event.target.value;

		setEquations(data);
	}

	function addEquation() {
		setEquations([...equations, ""]);
	}

	function removeEquation(idx: number) {
		let data = [...equations];
		data.splice(idx, 1);
		setEquations(data);
	}

	function generateEquations(equations: string[]) {


		if (!wasmLoaded) {
			return;
		}

		console.log(equations);

		let evaluator = new Evaluator();

		let out = equations.map((eq: string, idx: number) => {
			let response = undefined;
	

			console.log("boutta evaluate: " + eq);

			response = evaluator.evaluate(eq);
			console.log(response);

			return (
				<div className="equation" key={idx}>
				<input
					//contentEditable={true}
					placeholder="equation" 
					value={eq}
					onChange={event => handleEquationChange(idx, event)}
				/>

				<button onClick={() => removeEquation(idx)}>X</button>

				{response != undefined &&
				<div className="result">
					<p>= {response}</p>
				</div>
				}
			</div>
			);
		})

		evaluator.free();
		return out;
	}

	return (
		<>
			{generateEquations(equations)}

			<button onClick={addEquation} className="new_eq">+</button>

		</>
	);
}

export default EquationInput;
import { useEffect, useState } from "react";
import { Evaluator, EvaluatorResponse, evaluate_graph } from "./wasm-graph-calc/pkg/wasm_graph_calc.js";

function EquationInput({
		equations,
		setEquations,
		answers,

		wasmLoaded
	}: {
		equations: any, 
		setEquations: any,
		answers: any,

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

	function generateEquations(eqs: any, ans: any[]) {
		let out = ans.map((a, idx) => {
			return (
				<div className="equation" key={idx}>
				<input
					//contentEditable={true}
					placeholder="equation" 
					value={eqs[idx]}
					onChange={event => handleEquationChange(idx, event)}
				/>

				{/* <div className="differentiate">
					<p>differentiate</p>
					{/* <input type="checkbox" /> 
				</div> */}

				<button className="remove" onClick={() => removeEquation(idx)}>X</button>

				{a &&
				<div className="result">
					<p>{a.var_name} = {a.value}</p>
				</div>
				}
			</div>
			);
		})

		return out;
	}

	return (
		<>
			{generateEquations(equations, answers)}

			<button onClick={addEquation} className="new_eq">+</button>

		</>
	);
}

export default EquationInput;
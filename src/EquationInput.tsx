import { evaluate_string } from "./wasm-graph-calc/pkg/wasm_graph_calc.js";

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

	return (
		<>
			{equations.map((eq: string, idx: number) => {
				let result;
				
				if (wasmLoaded) {
					result = evaluate_string(eq);
				} else {
					result = undefined;
				}

				return (

					<div className="equation" key={idx}>
						<input
							placeholder="equation" 
							value={eq}
							onChange={event => handleEquationChange(idx, event)}
						/>

						<button onClick={() => removeEquation(idx)}>X</button>

						{result != undefined &&
						<div className="result">
							<p>= {result}</p>
						</div>
						}
					</div>
				);
			})}

			<button onClick={addEquation} className="new_eq">+</button>

		</>
	);
}

export default EquationInput;
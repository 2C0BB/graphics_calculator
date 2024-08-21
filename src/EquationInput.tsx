function EquationInput({
		equations,
		setEquations,
	}: {
		equations: any, 
		setEquations: any,
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
				return (

					<div className="equation" key={idx}>
						<input
							placeholder="equation" 
							value={eq}
							onChange={event => handleEquationChange(idx, event)}
						/>

						<button onClick={() => removeEquation(idx)}>X</button>
					</div>
				);
			})}

			<button onClick={addEquation} className="new_eq">+</button>

		</>
	);
}

export default EquationInput;
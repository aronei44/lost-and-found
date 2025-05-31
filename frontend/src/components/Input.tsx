const Input = (p: {
    type?: string;
    placeholder?: string;
    id?: string;
    required?: boolean;
    label?: string;
    onChange?: (e: React.ChangeEvent<HTMLInputElement>) => void;
    value?: string;
}) => {
    return (
        <div className="mb-6">
            <label 
                htmlFor={p.id} 
                className="block text-gray-700">
                    {p.label}
            </label>
            <input 
                type={p.type} 
                id={p.id} 
                name={p.id} 
                className="w-full p-2 border rounded" 
                required={p.required} 
                placeholder={p.placeholder} 
                onChange={p.onChange} 
                value={p.value} 
                autoComplete="off"
            />
        </div>
    )
}

export default Input;
export function Button(props: React.ButtonHTMLAttributes<HTMLButtonElement>) {
    return (
        <button type="button" {...props} className="bg-gray-500 py-1 px-2 rounded-sm">
            {props.children}
        </button>
    )
}
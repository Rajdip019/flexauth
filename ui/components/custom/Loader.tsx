export const Loader = ({ size = '1.25em' }) => {
    return (
        <div
            className={`inline-block animate-spin rounded-full border-2 border-solid border-current border-r-transparent align-[-0.125em] text-info motion-reduce:animate-[spin_1.5s_linear_infinite]`}
            role="status"
            style={{ width: size, height: size }}
        >
            <span
                className="absolute -m-px h-px w-px overflow-hidden whitespace-nowrap border-0 p-0 clip-[rect(0,0,0,0)]"
            >
                Loading...
            </span>
        </div>
    );
};

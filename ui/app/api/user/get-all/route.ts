export async function GET(req: Request) {    
    const endPoint: (string | undefined) = `${process.env.NEXT_PUBLIC_API_BASE_URL}/api/user/get-all`

    console.log('endPoint', endPoint);
    console.log('process.env.X_API_KEY', process.env.X_API_KEY);
    
    
    if (endPoint) {
        try {
            console.log('Fetching Users');
            
            const res = await fetch(endPoint, {
                headers: {
                    'Content-Type': 'application/json', // Set the appropriate content type for your request
                    'x-api-key': process.env.X_API_KEY!,
                },
            });
            const data = await res.json();
            return Response.json({ data })
        } catch (error) {
            console.error('Error during request:', error);
        }
    }
}
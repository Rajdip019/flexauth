export async function GET(req: Request): Promise<any> {
    const endPoint: (string | undefined) = `${process.env.NEXT_PUBLIC_API_BASE_URL}/user/get-all`

    if (endPoint) {
        try {
            const response = await fetch(endPoint, {
                method: 'GET',
                headers: {
                    'Content-Type': 'application/json', // Set the appropriate content type for your request
                    'x-api-key': process.env.X_API_KEY!,
                },
            });

            // if (!response.ok) {
            //     throw new Error('Network response was not ok');
            // }
            // If the response is successful, you can handle the result here
            const result = await response.json();
            console.log('POST request successful for Fetching Users');
            return Response.json({ result })
        } catch (error) {
            console.error('Error during POST request:', error);
        }
    }
}
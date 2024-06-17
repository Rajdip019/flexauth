export async function POST(req: Request) {
    const endPoint: (string | undefined) = `${process.env.NEXT_PUBLIC_API_BASE_URL}/api/password/reset`;

    const { email, old_password, new_password } = await req.json();

    if (endPoint) {
        try {
            const res = await fetch(endPoint, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json', // Set the appropriate content type for your request
                    'x-api-key': process.env.X_API_KEY!,
                },
                body: JSON.stringify({
                    email,
                    old_password,
                    new_password
                }),
                cache: 'no-cache',
            });
            const data = await res.json();
            return Response.json({ data })
        } catch (error) {
            console.error('Error during request:', error);
        }
    }
}
export async function fetchWithAuth(url: string, redirect = true, additional_headers = {}): Promise<Response> {
  const secret = localStorage.getItem('secret');
  let response;

  if (!secret) { // Auth might not be enabled on the server side
    response = await fetch(url);
  } else {
    const credentials = btoa(`default:${secret}`); // Using the built-in btoa function
    const headers = {
      Authorization: `Basic ${credentials}`,
      ...additional_headers,
    };

    response = await fetch(url, { headers });
  }

  if (redirect && response.status === 401) {
    window.location.href = '/login';
  }

  return response;
}

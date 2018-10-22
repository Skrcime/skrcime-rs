<login-form>
    <form class="form-group" action="/api/session" method="post" onsubmit="{ login }">
        <label class="form-label" for="input-email">Email</label>
        <input class="form-input" type="email" ref="email" placeholder="Email">

        <label class="form-label" for="input-password">Geslo</label>
        <input class="form-input" type="password" ref="password" placeholder="Geslo">

        <button class="btn btn-primary mt-2 float-right">Prijava</button>
    </form>

    <script>
        login(e) {
            e.preventDefault()

            const email = this.refs.email.value
            const password = this.refs.password.value

            fetch('/api/session', {
                method: 'POST',
                credentials: 'same-origin',
                headers: new Headers({ 'Content-Type': 'application/json' }),
                body: JSON.stringify({ email, password }),
            }).then(res =>
                res.json().then(json => {
                    if (!res.ok) return Promise.reject(json)
                    window.location = '//moj.skrci.me'
                })
            ).catch(err => {
                console.error('Error', err)
            })
        }
    </script>
</login-form>
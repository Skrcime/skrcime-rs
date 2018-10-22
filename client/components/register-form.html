<register-form>
    <form class="form-group" action="/api/users" method="post" onsubmit="{ register }">
        <label class="form-label" for="input-name">Ime</label>
        <input class="form-input" type="text" ref="name" placeholder="Ime">

        <label class="form-label" for="input-email">Email</label>
        <input class="form-input" type="email" ref="email" placeholder="Email">

        <label class="form-label" for="input-password">Geslo</label>
        <input class="form-input" type="password" ref="password" placeholder="Geslo">

        <button class="btn btn-primary mt-2 float-right">Registracija</button>
    </form>

    <script>
        register(e) {
            e.preventDefault()

            const name = this.refs.name.value
            const email = this.refs.email.value
            const password = this.refs.password.value

            fetch('/api/users', {
                method: 'POST',
                credentials: 'same-origin',
                headers: new Headers({ 'Content-Type': 'application/json' }),
                body: JSON.stringify({ name, email, password }),
            }).then(res =>
                res.json().then(json => {
                    if (!res.ok) return Promise.reject(json)
                    window.location = '/prijava'
                })
            ).catch(err => {
                console.error('Error', err)
            })
        }
    </script>
</register-form>
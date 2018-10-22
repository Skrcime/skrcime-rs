<shorten-url>
    <form class="input-group" onsubmit="{ shorten }">
        <input ref="target_url" type="text" class="form-input" placeholder="Vnesi URL naslov">
        <button class="btn btn-primary input-group-btn">Skrci</button>
    </form>

    <script>
        shorten(e) {
            e.preventDefault()

            const target = this.refs.target_url.value

            fetch('/api/urls', {
                method: 'POST',
                credentials: 'same-origin',
                headers: new Headers({ 'Content-Type': 'application/json' }),
                body: JSON.stringify({ target }),
            }).then(res =>
                res.json().then(json => {
                    if (!res.ok) return Promise.reject(json)
                    console.log(json)
                    alert('Ok!')
                })
            ).catch(err => {
                console.error('Error', err)
            })
        }
    </script>
</shorten-url>
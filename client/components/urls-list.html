<urls-list>
    <ol class="container">
        <li each={ urls } class="columns">
            <a href="http://skrci.me/{ hash }" class="column col-3">skrci.me/{ hash }</a>
            <span class="column col-3">{ target }</span>
        </div>
    </ol>

    <script>
        this.urls = []

        getUrls() {
            fetch('/api/urls', {
                method: 'GET',
                credentials: 'same-origin',
                headers: new Headers({ 'Content-Type': 'application/json' })
            }).then(res =>
                res.json().then(json => {
                    if (!res.ok) return Promise.reject(json)
                    this.urls = json
                    this.update()
                })
            ).catch(err => {
                console.error('Error', err)
            })
        }

        this.on('mount', () => {
            this.getUrls()
        })
    </script>

    <style>
        li {
            list-style-type: none;
        }
    </style>
</urls-list>
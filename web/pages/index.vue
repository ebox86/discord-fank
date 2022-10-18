<template>
    <div>
        <header class="header">
            <div />
            <ul>
                <li>
                    <nuxt-link to="/">Home</nuxt-link>
                </li>
                <va-popover
                    message="Login not working yet :keku:"
                    color="#9f03e0"
                >
                    <va-button color="#9f03e0">Login</va-button>
                </va-popover>
            </ul>
        </header>
        <div class="content-wrap">
            <div class="hero">
                <img src="~/assets/logo.png" alt="Fankbot Logo" />
                <h1>a discord bot..</h1>
                <h3>..for apes, by apes</h3>
                <br/>
                <va-button class="mr-4 mb-2" disabled color="#9f03e0">Add to Discord</va-button>
            </div>
        </div>
        <div class="section">
            <div class="feature-list">
                <div class="row">
                    <div class="flex md6 lg4 vcard">
                    <va-card color="#9f03e0" gradient>
                        <va-card-title>Watchlists</va-card-title>
                        <va-card-content>Keep track of and show off custom watchlists with buildin data from IEX Cloud.</va-card-content>
                    </va-card>
                    </div>
                    <div class="flex md6 lg4 vcard">
                    <va-card color="#9f03e0" gradient>
                        <va-card-title>Rank System</va-card-title>
                        <va-card-content>Fank keeps track of member rank based on several different server activities.</va-card-content>
                    </va-card>
                    </div>
                    <div class="flex md6 lg4 vcard">
                    <va-card color="#9f03e0" gradient >
                        <va-card-title>Competitions</va-card-title>
                        <va-card-content>Define and run competitions from within Fank. Let members register and fank takes care of the rest.</va-card-content>
                    </va-card>
                    </div>
                    <div class="flex md6 lg4 vcard">
                    <va-card color="#9f03e0" gradient >
                        <va-card-title>Voice Callouts</va-card-title>
                        <va-card-content>Configure levels and pivots and let Fank call them out in one of your VC channels.</va-card-content>
                    </va-card>
                    </div>
                </div>
            </div>
            <div>
                <h2>What is Fankbot?</h2>
            </div>
            <div>
                <p>
                    Fankbot is a discord bot that is made for apes, by apes. It is a community driven project that is open source and free to use. It is currently in development and will be released soon.
                </p>
            </div>
        </div>

    </div>
</template>

<script>
export default {}
</script>

<style>

.vcard {
    padding: 1rem;
}
.section {
    margin-bottom: 50px;
    background: hsla(0,0%,100%,.4);
    padding: 40px 80px;
    border: 3px solid rgba(0,0,0,.1);
    border-left: 0;
    border-right: 0;
}
.hero {
    font-size: 26px;
    line-height: 1.4;
    font-family: "Veggy";
    font-weight: 300;
    box-sizing: border-box;
    text-align: center;
    padding: 40px 0;
    background: rgb(0, 0, 0);
    background-size: 16px 16px;
    background-color: black;
    color: rgb(255, 255, 255);
}
.feature-list {
    display: -webkit-box;
    display: -ms-flexbox;
    display: flex;
    -ms-flex-wrap: wrap;
    flex-wrap: wrap;
}

.content-wrap {
    line-height: 1.4;
    color: #555;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
          Oxygen, Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
    font-weight: 300;
    box-sizing: border-box;
    -webkit-box-flex: 1;
    flex: 1;
    position: relative;
}
    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;

        /* border-bottom: 5px solid rgb(255, 0, 0); */
        background-color: #000;
        color: #fff;
        padding: 10px;
    }

    .header .title {
        font-size: 3rem;
        font-weight: bold;
    }

    .header ul {
        display: flex;
        list-style: none;
        margin: 0;
        padding: 0;
        padding-right: 5rem;
    }
    .h1 {
    font-family: "Veggy";
    letter-spacing: 1px;
    word-spacing: 3px;
    }

    .header a {
        color: #ccc;
        display: inline-block;
        padding: 0.6rem 1rem;
        margin-right: 0.5rem;
    }
</style>
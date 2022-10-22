<script>

export default  {
  mounted() {
    if(this.$device.isDesktop)
    document.addEventListener('mousemove', parallax)
    function parallax(event) {
        this.querySelectorAll(".parallax-wrap span").forEach((shift) => {
            const position = shift.getAttribute("value");
            const x = (window.innerWidth - event.pageX * position) / 90;
            const y = (window.innerHeight - event.pageY * position) / 90;

            shift.style.transform = `translateX(${x}px) translateY(${y}px)`;
            });
        }
    }
}
</script>
<template>
    <NuxtLayout>
        <div class="content-wrap">
            <div class="hero">
                <div class="parallax-wrap">
                <span value="-5">
                    <img src="../static/img/shape1.png" />
                </span>
                <span value="5">
                    <img src="../static/img/shape2.png" />
                </span>
                <span value="-12">
                    <img src="../static/img/shape3.png" />
                </span>
                <span value="7">
                    <img src="../static/img/shape4.png" />
                </span>
                <span value="-26">
                    <img src="../static/img/shape5.png" />
                </span>
                <img src="~/assets/logo.png" alt="Fankbot Logo" />
                <h2>a discord bot..</h2>
                <h3>..for apes, by apes</h3>
                <br/>
                <va-button class="mr-4 mb-2" disabled color="#9f03e0">Add to Discord</va-button>
                </div>
            </div>
        </div>
        <div class="section">
            <div class="feature-list">
                <div class="row">
                    <div class="flex md6 lg4 vcard">
                    <va-card color="#9f03e0" gradient stripe stripe-color="success">
                        <va-card-title>Watchlists</va-card-title>
                        <va-card-content>Keep track of and show off custom watchlists with buildin data from IEX Cloud.</va-card-content>
                    </va-card>
                    </div>
                    <div class="flex md6 lg4 vcard">
                    <va-card color="#9f03e0" gradient stripe stripe-color="success">
                        <va-card-title>Rank System</va-card-title>
                        <va-card-content>Fank keeps track of member rank based on several different server activities.</va-card-content>
                    </va-card>
                    </div>
                    <div class="flex md6 lg4 vcard">
                    <va-card color="#9f03e0" gradient stripe stripe-color="success">
                        <va-card-title>Competitions</va-card-title>
                        <va-card-content>Define and run competitions from within Fank. Let members register and fank takes care of the rest.</va-card-content>
                    </va-card>
                    </div>
                    <div class="flex md6 lg4 vcard">
                    <va-card color="#9f03e0" gradient stripe stripe-color="success" disabled>
                        <va-card-title>Voice Callouts (coming soon)</va-card-title>
                        <va-card-content>Configure levels and pivots and let Fank call them out in one of your VC channels.</va-card-content>
                    </va-card>
                    </div>
                </div>
            </div>
            <div class="sub-section">
                <div class="icon">
                    <va-icon name="help" color="black" size="large" />
                </div>
                <div class="sub1">
                    <div>
                        <h2 class="text-xl">What is Fankbot?</h2>
                    </div>
                    <div>
                        <p>
                            Fankbot is a discord bot that is made for apes, by apes. It is a community driven project that is open source and free to use. It is currently in development and will be released soon.
                        </p>
                    </div>
                </div>
            </div>
        </div>
    </NuxtLayout>
</template>

<style>
    .vcard {
        padding: 0.25rem;
    }
    .section {
        margin-bottom: 50px;
        background: hsla(0,0%,100%,.4);
        padding: 40px 80px;
        border: 10px solid rgba(0,0,0,.1);
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
        padding-bottom: 2rem;

    }

    .sub-section {
        flex-direction: row;
        display: flex;
        padding-bottom: 2rem;
    }
    .icon {
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 1rem;
    }
    .sub1 {
        display: flex;
        flex-direction: column;
    }

    .content-wrap {
        line-height: 1.4;
        color: #555;
        font-family: "Veggy";
        font-weight: 300;
        box-sizing: border-box;
        -webkit-box-flex: 1;
        flex: 1;
        position: relative;
    }
    .parallax-wrap {
        position: relative;
        width: 100%;
        height: 500px;
        overflow: hidden;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }

    .parallax-wrap span {
        position: absolute;
    }
    .parallax-wrap span:nth-child(1) {
        top: 60%;
        left: 55%;
        z-index: 3;
    }
    .parallax-wrap span:nth-child(2) {
        top: 75%;
        left: 5%;
        z-index: 3;
    }
    .parallax-wrap span:nth-child(3) {
        top: -5%;
        left: 10%;
        z-index: 3;
    }
    .parallax-wrap span:nth-child(4) {
        top: 20%;
        left: 75%;
        z-index: 3;
    }
    .parallax-wrap span:nth-child(5) {
        top: -10%;
        left: 50%;
        z-index: 3;
    }
</style>
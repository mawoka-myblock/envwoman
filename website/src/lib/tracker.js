import Plausible from "plausible-tracker";

export const {trackPageview, enableAutoOutboundTracking, trackEvent} = Plausible({
    domain: 'envwoman.mawoka.eu',
    apiHost: "https://sugar.mawoka.eu.org"
})

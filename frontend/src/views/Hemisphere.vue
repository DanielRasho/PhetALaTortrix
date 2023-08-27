<template>
    <main>
        <div class="canvas" id="canvas"></div>
        <fieldsContainer :params="fields" @hot-reload="hotReloading()"></fieldsContainer>
    </main>
</template>

<script setup>
import fieldsContainer from '@/components/organism/fieldsContainer.vue'
import Two from 'two.js';
import { onMounted, ref } from 'vue'
import { initializePlane } from "../lib/plane"
import { HALF_PI } from 'two.js/src/utils/math';

const fields = ref({
    axis: {
        x: [-5, 25],
        y: [-5, 25]
    },
    figure: {
        radius: {
            value: 32,
            name: 'Radius',
            unit: 'm'
        },
        Charge: {
            value: 64,
            name: 'Charge',
            unit: "nC"
        }
    },
    points: []
});

/**
 * Draws the hemisphere into the screen.
 * @param {Two} drawer TwoJS object
 * @param {Number} columns number of columns of x-axis
 * @returns {[Two.Text, Two.Text]} the text elements that display info about the hemisphere
 */
const drawHemisphere = (drawer, columns) => {
    let columnGap = drawer.width / columns;

    let originPos = [columnGap*3 - columnGap/2, drawer.height / 2];

    let arcRadius = 100;
    let figureColor = "blue";
    let arc = drawer.makeArcSegment(originPos[0],originPos[1], arcRadius, arcRadius, HALF_PI, HALF_PI * 3);
    arc.stroke = figureColor;
    arc.linewidth = 3;

    let line = drawer.makeLine(originPos[0], originPos[1] - arcRadius, originPos[0], originPos[1] + arcRadius);
    line.stroke = figureColor;
    line.linewidth = 3;

    let circle = drawer.makeCircle(originPos[0] - arcRadius / 2, originPos[1], arcRadius / 2);
    circle.scale = new Two.Vector(1, 0.4);
    circle.stroke = figureColor;
    circle.linewidth = 4;
    circle.dashes = [4,5];
    circle.fill = "transparent";

    let rText = drawer.makeText("R = 0 m", originPos[0] + 5, originPos[1] - arcRadius, {
        alignment: "left"
    });
    let cText = drawer.makeText("Q = 0 nC", originPos[0] - arcRadius / 2, originPos[1] + arcRadius / 4, {
        fill: "red",
        stroke: 10,
    });

    return [rText, cText];
};

let radiusText;
let chargeText;

onMounted(() => {
    let elem = document.querySelector("#canvas");
    let two = new Two({ fullscreen: false, width: elem.offsetWidth, height: elem.offsetHeight }).appendTo(elem);
    let rows = 11;
    let columns = 25;

    initializePlane(two, columns, rows);
    [radiusText, chargeText] = drawHemisphere(two, columns, rows);

    two.update();
})

</script>
<style scoped>
main {
    flex-grow: 1;
    padding: 3ch;
}
.canvas {
    display: block;
    background-color: white;
    border-radius: 7px;
    min-height: 55vh;
    margin-bottom: 3ch;
}
</style>

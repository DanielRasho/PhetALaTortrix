<template>
    <main>
        <div id="canvas" class="canvas"></div>
        <fieldsContainer
         :fields="fields"
         @changesSubmited="updateFields($event)"
        ></fieldsContainer>
    </main>
</template>

<script setup>
import fieldsContainer from '@/components/organism/fieldsContainer.vue'
import { onMounted, ref, toRaw } from 'vue';
import Two from 'two.js';
import { HALF_PI } from 'two.js/src/utils/math';
import { initializePlane } from '../lib/plane.js';
import { js_hemisphere_field_on } from 'tphet';
import { Vector } from 'two.js/src/vector';

let two;

onMounted(() => {
    let elem = document.querySelector("#canvas");
    two = new Two({ fullscreen: false, width: elem.offsetWidth, height: elem.offsetHeight }).appendTo(elem);
    drawCanvas(fields.value);
    two.update();
})

const fields = ref({
    axis: {
        x: {
            min: -5,
            max: 5
        }
    },
    figure: {
        radius: {
            value: 1,
            name: 'Radius',
            unit: 'm'
        },
        charge: {
            value: 1,
            name: 'Charge',
            unit: 'nC'
        },
    },
    points: []
})

function updateFields(newValue) {
    fields.value = newValue
    console.log("FROM PARENT");
    console.dir(fields.value);
    two.clear();
    drawCanvas(toRaw(newValue))
}

/**
 * Draws the hemisphere into the screen.
 * @param {Two} drawer TwoJS object
 * @param {Number} columns number of columns of x-axis
 * @param {Object} context object that contains all the fields that the user can change.
 */
 const drawHemisphere = (drawer, columns, context) => {
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

    let rString = `R = ${context.figure.radius.value} m`;
    drawer.makeText(rString, originPos[0] + 5, originPos[1] - arcRadius, {
        alignment: "left"
    });

    let cString = `Q = ${context.figure.charge.value} nC`
    drawer.makeText(cString, originPos[0] - arcRadius / 2, originPos[1] + arcRadius / 4, {
        fill: "red",
        stroke: 10,
    });
};

/**
 * Draws the hemisphere into the screen.
 * @param {Two} drawer TwoJS object
 * @param {Object} context object that contains all the fields that the user can change.
 */
const drawPoints = (drawer, context) => {
    let {min, max} = context.axis.x;
    let virtualWidth = max-min;
    let originY = drawer.height / 2;
    let figure = {
        radius: context.figure.radius.value,
        charge: context.figure.charge.value,
    };

    let vectors = context.points.map((point) => new Two.Vector((point - min) / virtualWidth * drawer.width, originY));
    console.log("Points drawed");
    vectors.forEach(v => {
        let c = drawer.makeCircle(v.x, v.y, 4);
        c.fill = "red";
    });

    // vectors.forEach(v => {
    //     try {
    //         let field = js_hemisphere_field_on(figure, v.x / two.width * virtualWidth, 250);
    //         let x = field / virtualWidth * drawer.width;
    //         drawer.makeArrow(v.x, v.y, x, v.y);   
    //     } catch (ex) {
    //         console.log("Error creating arrows...");
    //         console.error(ex);
    //     }
    // });
};

function drawCanvas(context){
    let rows = 11;
    let columns = 25;

    initializePlane(two, columns, rows);
    drawHemisphere(two, columns, context);
    drawPoints(two, context);

    two.update();
}

</script>

<style scoped>
main {
    grid-area: 'main';
    padding: 3ch;
}
.canvas {
    display: block;
    background-color:white;
    border-radius: 7px;
    min-height: 55vh;
    margin-bottom: 1ch;
}
</style>
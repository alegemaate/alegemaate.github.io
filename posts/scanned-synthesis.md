---
title: Scanned Synthesis using a One-Dimensional String
date: 2026-05-05
description: Diving into Scanned Synthesis, a powerful and under-explored audio synthesis and performance technique.
---

Scanned synthesis is not something that gets a lot of attention in a world dominated by subtractive, wavetable, and additive synthesis. To me, this makes it all the more interesting as there is a lot to explore in terms of sound design and performance techniques. Conceptually, it is one of the most elegant synthesis techniques that I have come across.

At its core, scanned synthesis is a physical modelling technique that simulates the behaviour of a slowly vibrating object (such as a string or a membrane) and then "scans" it periodically to produce sound. The scanning process involves sampling the state of the vibrating object at regular intervals along a path that is independent of the physical simulation to produce a desired pitch.

Timbre is instead determined by the physical properties of the vibrating object such as mass, stiffness, and damping. By changing these properties, a wide variety of sounds can be produced that are difficult or impossible with other synthesis techniques.

Since scanned synthesis separates pitch from timbre, and does not define the underlying model, it allows arbitrary models to be selected. One relatively simple model is a one-dimensional string [Verplank et al. 2001].

## One-Dimensional String

Imagine a flexible string connected in a loop. The string is made up of a series of masses, each connected to its two neighbours by springs. Additionally (and physically impossibly) each mass is connected to a fixed reference point by a spring and a damper. If you pluck the string, waves propagate along it, interacting with each other to create complex vibrations. In scanned synthesis, that shape is then scanned periodically to produce the audible waveform.

When a string is excited by an external force, the waves that propagate along it are determined by the physical properties of the string, such as the mass of each node, the stiffness of the springs, and the damping effects of the dampers. These obey Newton's laws, so can be easily conceptualised using concepts from the real world. This gives it a unique character that is difficult to achieve with other synthesis techniques. By changing the properties of the string, we can create a wide variety of sounds.

A continuous analogue of this model can be written as a damped wave equation with an additional restoring term to ground. This is based on Hooke's law and Newton's second law:

$$
\frac{\partial^2 u}{\partial t^2} = c^2 \frac{\partial^2 u}{\partial x^2} - \alpha \frac{\partial u}{\partial t} - \beta u
$$

Where:

- $u(x, t)$ is the displacement of the string at position $x$ and time $t$.
- $c$ is the wave speed, which is related to the stiffness between neighbouring masses and the mass of each node.
- $\alpha$ is an effective damping coefficient, related to the damping-to-earth coefficient and mass.
- $\beta$ is an effective restoring coefficient, related to the stiffness-to-earth coefficient and mass.

For simulation purposes, we are interested in the discrete version of this equation, which can be expressed as:

$$
x_i'' = \frac{( k_n (x_{i-1} - 2x_i + x_{i+1}) - k_e x_i - c_e x_i')}{m}
$$

Where:

- **Mass $m$**: The mass given to each node, which determines inertia.

- **Stiffness to Neighbours $k_n$**: The stiffness of the springs connecting each mass to the neighbouring masses. This determines how much the string resists deformation and thus how quickly waves propagate along it.

- **Damping to Earth $c_e$**: The damping coefficient of the dampers connecting each mass to the ground. This determines how much energy is lost as the string vibrates, affecting the sustain and decay of the sound.

- **Stiffness to Earth $k_e$**: The stiffness of the springs connecting each mass to the ground. This determines how much force is applied to the mass to return it to its resting position (usually position 0).

- **Displacement $x_i$**: The current position of each node on the string, which changes over time as the string vibrates.

> For a breakdown of this equation, see the "Deriving the Discrete String Model" section below.

All of these parameters can be uniform or non-uniform across the string.

### Simulation

Try dragging the nodes to excite the string, and adjusting the sliders to change the parameters of the string.

```js {jsxgraph=true, height=250}
let board = JXG.JSXGraph.initBoard(BOARDID, {
  boundingbox: [-10.5, 4, 10.5, -4],
  keepaspectratio: false,
  axis: false,
  grid: false,
  showNavigation: false,
  showCopyright: false,
  resize: {
    enabled: true,
  },
});
let numNodes = 24;
let positions = [];
let velocities = [];
let nodes = [];
let segments = [];
let draggedNode = -1;
let i;

for (i = 0; i < numNodes; i++) {
  positions.push(0);
  velocities.push(0);
}

function nodeX(idx) {
  return -9 + idx * (18 / (numNodes - 1));
}

for (i = 0; i < numNodes; i++) {
  (function (idx) {
    var vLine = board.create(
      "line",
      [
        [nodeX(idx), -100],
        [nodeX(idx), 100],
      ],
      {
        visible: false,
        straightFirst: false,
        straightLast: false,
        fixed: true,
      },
    );

    var pt = board.create("glider", [nodeX(idx), 0, vLine], {
      name: "",
      withLabel: false,
      size: 2,
      strokeColor: "#333",
      fillColor: "#333",
    });

    pt.on("down", function () {
      draggedNode = idx;
    });

    nodes.push(pt);
  })(i);
}

for (i = 0; i < nodes.length - 1; i++) {
  const segment = board.create("segment", [nodes[i], nodes[i + 1]], {
    strokeColor: "#666",
    strokeWidth: 1,
    fixed: true,
  });
  segments.push(segment);
}

let k_n = 200;
let k_e = 10;
let c_e = 0.3;
let m = 1;
let dt = 0.002;
let subSteps = 8;

// Sliders
let k_n_slider = board.create(
  "slider",
  [
    [-10, -0.5],
    [-6, -0.5],
    [0, k_n, 300],
  ],
  {
    name: "stiff to neighbours (k_n)",
    size: 2,
  },
);
k_n_slider.on("drag", function () {
  k_n = k_n_slider.Value();
});

let k_e_slider = board.create(
  "slider",
  [
    [-10, -1.5],
    [-6, -1.5],
    [0, k_e, 100],
  ],
  {
    name: "stiff to earth (k_e)",
    size: 2,
  },
);
k_e_slider.on("drag", function () {
  k_e = k_e_slider.Value();
});

let c_e_slider = board.create(
  "slider",
  [
    [-10, -2.5],
    [-6, -2.5],
    [0, c_e, 5],
  ],
  {
    name: "damping (c_e)",
    size: 2,
  },
);
c_e_slider.on("drag", function () {
  c_e = c_e_slider.Value();
});

let m_slider = board.create(
  "slider",
  [
    [-10, -3.5],
    [-6, -3.5],
    [0.1, m, 10],
  ],
  {
    name: "mass",
    size: 2,
  },
);
m_slider.on("drag", function () {
  m = m_slider.Value();
});

// Reset button
board.create("button", [
  -10,
  3,
  "Reset",
  function () {
    for (var j = 0; j < nodes.length; j++) {
      positions[j] = 0;
      velocities[j] = 0;
      nodes[j].setPosition(JXG.COORDS_BY_USER, [nodeX(j), 0]);
    }
  },
]);

function physicsStep() {
  if (board.mode !== board.BOARD_MODE_DRAG) {
    draggedNode = -1;
  }

  if (draggedNode >= 0) {
    positions[draggedNode] = nodes[draggedNode].Y();
    velocities[draggedNode] = 0;
  }

  for (var s = 0; s < subSteps; s++) {
    var accel = [];
    for (var j = 0; j < nodes.length; j++) {
      var left = positions[(j + nodes.length - 1) % nodes.length];
      var right = positions[(j + 1) % nodes.length];
      accel[j] =
        (k_n * (left - 2 * positions[j] + right) -
          k_e * positions[j] -
          c_e * velocities[j]) /
        m;
    }
    for (var j2 = 0; j2 < nodes.length; j2++) {
      if (j2 === draggedNode) continue;
      velocities[j2] += accel[j2] * dt;
      positions[j2] += velocities[j2] * dt;
    }
  }

  board.suspendUpdate();
  for (var k = 0; k < nodes.length; k++) {
    if (k === draggedNode) {
      continue;
    }
    nodes[k].setPosition(JXG.COORDS_BY_USER, [nodeX(k), positions[k]]);
  }

  // Sizing
  for (let i = 0; i < nodes.length; i++) {
    nodes[i].setAttribute({
      size: 1 + m * 0.5,
    });
  }

  for (let i = 0; i < segments.length; i++) {
    segments[i].setAttribute({
      strokeWidth: 1 + k_n * 0.02,
    });
  }

  board.unsuspendUpdate();
  board.update();
}

setInterval(physicsStep, 30);
```

### Simulating the String in Code

In code, the string can be simulated by discretizing it into a series of nodes and updating their positions over time according to the string equation.

```rust
fn update_string(
    position: &mut [f32],
    velocity: &mut [f32],
    mass: f32,
    k_neighbour: f32,
    c_earth: f32,
    k_earth: f32,
    dt: f32,
) {
    let n = position.len();
    let mut acceleration = vec![0.0; n];

    for i in 0..n {
        let left = position[(i + n - 1) % n];
        let right = position[(i + 1) % n];

        let neighbour_force =
            k_neighbour * (left - 2.0 * position[i] + right);

        let earth_spring_force =
            -k_earth * position[i];

        let earth_damping_force =
            -c_earth * velocity[i];

        let force =
            neighbour_force + earth_spring_force + earth_damping_force;

        acceleration[i] = force / mass;
    }

    for i in 0..n {
        velocity[i] += acceleration[i] * dt;
        position[i] += velocity[i] * dt;
    }
}
```

> Note that in the visualisation above, you cannot see the first and last node being connected. However, they are connected in the simulation. Try dragging the first or last node to see the propagation of the wave around the loop.

## Excitation

The string model is static at rest, so it must be excited by an external force to produce sound. In the example above, this is a gesture with the mouse acting on a single node. For a basic use case, this may be interesting enough. However, we can push it further.

As a method of excitation, one can vary the force applied, the duration of the force, or the location of the force. The way the string is excited will affect the resulting sound, allowing for a wide range of expressive possibilities.

Some ways I can imagine exciting the string include:

**Single Node Displacement**: Simulated by moving a single node away from the rest and then releasing it.

**Single Node Force**: Simulated by applying a short force impulse to a single node.

**Hammering Motion**: A hammering motion can be simulated by applying a force to a node as a function of time, such as a short burst of force followed by a release.

**Bowing Motion**: A bowing motion can be simulated by applying a continuous force to a node on the string. The force can be modulated over time to simulate the changing pressure of a bow on the string.

**Strumming Motion**: A strumming motion can be simulated by applying forces to multiple nodes on the string in quick succession. The time interval can be modulated to simulate different strumming speeds.

**Wave Overlay**: By applying particular "shaped" forces to multiple nodes on the string in a coordinated manner, one can create complex wave patterns that evolve over time. For example, applying a sinusoidal force to a node can create a wave that propagates along the string, and by modulating the frequency and amplitude of the force, one can create interesting timbral effects.

**Continuous Modulation**: By continuously modulating the force applied to a node on the string, one can create evolving sounds that change over time. For example, applying a low-frequency oscillation (LFO) to the force can create a tremolo-like effect or timbral variation, while applying an envelope can create a dynamic sound that evolves over time.

You can imagine combining these, or coming up with totally new methods of excitation. The possibilities are vast, and the unique character of scanned synthesis allows for a wide range of expressive possibilities.

### Complex Gestures

The unique character of scanned synthesis, and a core part of its concept, is that it can be performed in real-time [Verplank et al. 2001]. The inputs to the system can include not only the parameters of the string, but also excitation gestures or scanning paths. This allows for a wide range of expressive possibilities, as the performer can interact with the string in a dynamic and intuitive way. Just as you can easily interact with the physical properties of a real string instrument, you can interact with the virtual string in scanned synthesis to create a wide variety of sounds and performance techniques.

## Scanning the String to Produce Sound

This is where the "scanned" part of scanned synthesis comes in. Instead of directly outputting the physical vibration of the string, we treat the displacement of each node as a "dynamic wavetable" and sample it at audio rate.

To produce a pitched sound, the scanner completes a loop around the string at the desired fundamental frequency. One complete trip around the string corresponds to one waveform cycle. For example, if the scanner completes one loop every 1/440 seconds, the resulting sound will have a fundamental frequency of 440 Hz, or A4. The string simulation itself may evolve more slowly, while the scanner reads from it at audio rate, causing the waveform shape to change over time.

If the desired output frequency is \(f\), and the audio sample rate is \(f_s\), then the scan phase advances by:

$$
\Delta p = \frac{fN}{f_s}
$$

where \(N\) is the number of nodes in the string.

At each audio sample, we read the string position at the current scan phase, preferably with interpolation:

$$
y[n] = x\_{\text{interp}}(p)
$$

> Without interpolation, the output can have stair-stepping, harsh artefacts, and aliasing, especially with a small node count or high scan frequencies. Interpolation is crucial for producing a smooth output by estimating the string's displacement at non-integer positions between nodes.

Then:

$$
p \leftarrow (p + \Delta p) \bmod N
$$

## How It Differs From Wavetable Synthesis

At a surface level, scanned synthesis can appear similar to wavetable synthesis because both techniques read through a table-like shape to produce a waveform. The _technique_ is similar, however Scanned Synthesis extends it by making the waveform the current state of a physical system. Wavetable synthesis, on the other hand, typically uses static waveforms that are precomputed and stored in memory.

In other words, the waveform is not simply read from a static table. The table itself is ever-evolving based on the physical simulation of the string.

## Further Considerations

- The string can be extended to two dimensions to create a membrane, which can produce more complex sounds.
- The scanning path can be more complex than a simple loop, allowing for more interesting timbral possibilities. For example, the scanning path could be a pre-seeded random walk, or a path that is influenced by the state of the string itself, creating a feedback loop between the physical simulation and the scanning process.
- The parameters of the string can be modulated in real-time to create dynamic changes in the sound.

## Appendix

### Deriving the Discrete String Model

In order to fully understand the string equation, we can derive it from Hooke's law and Newton's second law.

Hooke's law states that the force exerted by a spring is proportional to the displacement of the spring from its resting position.

$$
F_\text{Hooke} = -kx
$$

Where $F_\text{Hooke}$ is the restoring force exerted by the spring, $k$ is the stiffness of the spring, and $x$ is the displacement of the spring from its resting position.

Newton's second law states that the force acting on an object is equal to the mass of the object multiplied by its acceleration:

$$
F = ma
$$

#### Single Mass to Earth with Damper

Imagine a single mass connected to earth by:

1. A spring with stiffness $k_\text{e}$.
2. A damper with damping coefficient $c_\text{e}$.

The spring force is:

$$
F_\text{spring} = -k_e x
$$

The damper force is:

$$
F_\text{damper} = -c_e x'
$$

Where:

- $x$ is the displacement of the mass from its resting position (usually position 0).
- $x'$ is the velocity of the mass.
- $x''$ is the acceleration of the mass.

Newton states that all forces acting on the mass must sum to the mass multiplied by its acceleration:

$$
m x'' = \sum F
$$

So,

$$
m x'' = F_\text{spring} + F_\text{damper}
$$

$$
m x'' = -k_e x - c_e x'
$$

That gives us the equation of motion for a single mass connected to earth by a spring and damper.

#### Full String with Neighbour Springs

In our theoretical string, we have not one, but two types of springs:

1. the springs connecting each mass to its neighbours
2. the spring (and damper) connecting each mass to earth

```text
x[i-1] === k_n === x[i] === k_n === x[i+1]
   |                 |                 |
 k_e,c_e           k_e,c_e           k_e,c_e
   |                 |                 |
 earth             earth             earth
```

Where:

- $k_e$ is spring-to-earth stiffness

- $c_e$ is damping-to-earth coefficient

- $k_n$ is neighbour spring stiffness

- $x_i$ is the displacement of mass $i$

The force from the neighbouring springs can be expressed as:

$$
F_{i,\text{left}} = k_n (x_{i-1} - x_i)
$$

$$
F_{i,\text{right}} = k_n (x_{i+1} - x_i)
$$

Combining all forces gives us:

$$
F_i = F_{i,\text{left}} + F_{i,\text{right}} + F_{i,\text{earth}} + F_{i,\text{damper}}
$$

$$
= k_n (x_{i-1} - x_i) + k_n (x_{i+1} - x_i) - k_e x_i - c_e x_i'
$$

$$
= k_n (x_{i-1} - 2x_i + x_{i+1}) - k_e x_i - c_e x_i'
$$

Substituting Newton's second law gives us:

$$
m x_i'' = k_n (x_{i-1} - 2x_i + x_{i+1}) - k_e x_i - c_e x_i'
$$

$$
x_i'' = \frac{( k_n (x_{i-1} - 2x_i + x_{i+1}) - k_e x_i - c_e x_i')}{m}
$$

This is the discrete equation we simulate directly in code. Each mass is updated according to the forces acting on it, which are determined by the positions of its neighbours and its own position and velocity.

### References

Verplank, Bill, et al. “Scanned Synthesis.” The Journal of the Acoustical Society of America, vol. 109, no. 5_Supplement, 1 May 2001, pp. 2400–2400, https://doi.org/10.1121/1.4744477. Accessed 10 Nov. 2025.

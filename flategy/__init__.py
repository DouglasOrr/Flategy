"""Flategy - a basic playable strategy game & bot."""

import os
import io
import subprocess
import tempfile

import cairo
import IPython.display
import numpy as np


class State:
    __slots__ = ['position', 'radius', 'world_shape']

    def __init__(self, position, radius, world_shape):
        self.position = position
        self.radius = radius
        self.world_shape = world_shape

    def to_dict(self):
        return dict(position=self.position,
                    radius=self.radius,
                    world_shape=self.world_shape)

    def replace(self, **args):
        d = self.to_dict()
        d.update(args)
        return type(self)(**d)

    @property
    def world_aspect(self):
        (left, top), (right, bottom) = self.world_shape
        return (bottom - top) / (right - left)

    # Rendering

    def draw(self, surface, width):
        ctx = cairo.Context(surface)
        # set up the basic view transformation
        (left, top), (right, bottom) = self.world_shape
        scale = width / (right - left)
        ctx.scale(scale, scale)
        ctx.translate(-left, -top)
        ctx.rectangle(left, top, right, bottom)
        ctx.set_source_rgb(255, 255, 255)
        ctx.fill()
        ctx.set_source_rgb(0, 0, 0)
        # render the world
        for pos, r in zip(self.position, self.radius):
            ctx.arc(pos[0], pos[1], r, 0, 2*np.pi)
            ctx.fill()

    def to_svg(self, width):
        f = io.BytesIO()
        with cairo.SVGSurface(f, width, int(self.world_aspect * width)) as surface:
            self.draw(surface, width)
        f.seek(0)
        return f.read()

    def _repr_svg_(self):
        return self.to_svg(256).decode('utf8')

    @classmethod
    def video(cls, states, filename, dt, width):
        with tempfile.TemporaryDirectory() as tmp:
            # Render PNG frames
            for n, frame in enumerate(states):
                with cairo.ImageSurface(cairo.FORMAT_ARGB32, width, int(frame.world_aspect * width)) as surface:
                    frame.draw(surface, width)
                    surface.write_to_png(os.path.join(tmp, 'frame_{:04d}.png'.format(n)))
            # Convert PNG* => MP4
            subprocess.check_call(['ffmpeg', '-i', os.path.join(tmp, 'frame_%04d.png'),
                                   '-y', '-r', str(int(1/dt)), '-pix_fmt', 'yuv420p', filename])
        return IPython.display.Video(filename)

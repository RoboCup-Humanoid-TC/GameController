package teamcomm.gui;

import com.jogamp.opengl.GL;
import com.jogamp.opengl.GL2;
import com.jogamp.opengl.GLAutoDrawable;
import com.jogamp.opengl.GLCapabilities;
import com.jogamp.opengl.GLEventListener;
import com.jogamp.opengl.GLProfile;
import com.jogamp.opengl.awt.GLCanvas;
import com.jogamp.opengl.glu.GLU;
import com.jogamp.opengl.util.Animator;
import java.awt.event.MouseEvent;
import java.awt.event.MouseWheelEvent;
import java.awt.event.MouseWheelListener;
import java.io.IOException;
import java.nio.FloatBuffer;
import java.util.Comparator;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Iterator;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;
import java.util.Set;
import javax.swing.JOptionPane;
import javax.swing.event.MouseInputAdapter;
import javax.xml.stream.XMLStreamException;
import teamcomm.PluginLoader;
import teamcomm.data.RobotData;
import teamcomm.data.RobotState;
import teamcomm.gui.drawings.Drawing;
import teamcomm.gui.drawings.Models;
import teamcomm.gui.drawings.PerPlayer;
import teamcomm.gui.drawings.Static;

/**
 *
 * @author Felix Thielke
 */
public class View3D implements GLEventListener {

    private static final float NEAR_PLANE = 1;
    private static final float FAR_PLANE = 20;

    private final GLCanvas canvas;
    private final Animator animator;

    private final int[] teamNumbers = new int[]{PluginLoader.TEAMNUMBER_COMMON, PluginLoader.TEAMNUMBER_COMMON};

    private final PriorityQueue<Drawing> drawings = new PriorityQueue<Drawing>(new Comparator<Drawing>() {
        @Override
        public int compare(final Drawing o1, final Drawing o2) {
            // opaque objects have priority over transparent objects
            if (o1.hasAlpha() && !o2.hasAlpha()) {
                return 1;
            }
            if (!o1.hasAlpha() && o2.hasAlpha()) {
                return -1;
            }

            // higher priorities are drawn earlier
            return o2.getPriority() - o1.getPriority();
        }
    });

    private final Map<String, Integer> objectLists = new HashMap<String, Integer>();

    private float cameraTheta = 45;
    private float cameraPhi = 0;
    private float cameraRadius = 9;

    private int width = 0;
    private int height = 0;

    public View3D() {
        // Initialize GL canvas and animator
        GLProfile glp = GLProfile.get(GLProfile.GL2);
        GLCapabilities caps = new GLCapabilities(glp);
        canvas = new GLCanvas(caps);
        canvas.addGLEventListener(this);
        animator = new Animator(canvas);

        // Setup camera movement
        final MouseInputAdapter listener = new MouseInputAdapter() {

            private int[] lastPos = null;

            @Override
            public void mousePressed(final MouseEvent e) {
                if (e.getButton() == MouseEvent.BUTTON1) {
                    lastPos = new int[]{e.getX(), e.getY()};
                }
            }

            @Override
            public void mouseReleased(final MouseEvent e) {
                if (e.getButton() == MouseEvent.BUTTON1) {
                    lastPos = null;
                }
            }

            @Override
            public void mouseDragged(final MouseEvent e) {
                if (lastPos != null) {
                    final float factor = 1.0f / 5.0f;
                    cameraPhi += (e.getX() - lastPos[0]) * factor;
                    if (cameraPhi < -90) {
                        cameraPhi = -90;
                    } else if (cameraPhi > 90) {
                        cameraPhi = 90;
                    }
                    cameraTheta -= (e.getY() - lastPos[1]) * factor;
                    if (cameraTheta < 0) {
                        cameraTheta = 0;
                    } else if (cameraTheta > 90) {
                        cameraTheta = 90;
                    }
                    lastPos = new int[]{e.getX(), e.getY()};
                }
            }

        };
        canvas.addMouseListener(listener);
        canvas.addMouseMotionListener(listener);
        canvas.addMouseWheelListener(new MouseWheelListener() {
            @Override
            public void mouseWheelMoved(final MouseWheelEvent e) {
                cameraRadius -= e.getWheelRotation() * 0.05;
                if (cameraRadius < NEAR_PLANE) {
                    cameraRadius = NEAR_PLANE;
                } else if (cameraRadius > FAR_PLANE - 5) {
                    cameraRadius = FAR_PLANE - 5;
                }
            }
        });

        // Setup common drawings
        drawings.addAll(PluginLoader.getInstance().getCommonDrawings());

        // Start rendering
        animator.start();
    }

    public void terminate() {
        animator.stop();
        canvas.destroy();
    }

    public GLCanvas getCanvas() {
        return canvas;
    }

    @Override
    public void init(final GLAutoDrawable glad) {
        final GL2 gl = glad.getGL().getGL2();

        // Enable VSync
        gl.setSwapInterval(1);

        // enable depth test
        gl.glClearDepth(1.0f);
        gl.glDepthFunc(GL.GL_LEQUAL);
        gl.glEnable(GL.GL_DEPTH_TEST);

        // avoid rendering the backside of surfaces
        gl.glEnable(GL.GL_CULL_FACE);
        gl.glCullFace(GL.GL_BACK);
        gl.glFrontFace(GL.GL_CCW);

        // Enable lightning, texturing and smooth shading
        gl.glEnable(GL2.GL_LIGHTING);
        gl.glEnable(GL.GL_MULTISAMPLE);
        gl.glEnable(GL.GL_TEXTURE_2D);
        gl.glPolygonMode(GL2.GL_FRONT, GL2.GL_FILL);
        gl.glShadeModel(GL2.GL_SMOOTH);
        gl.glColorMaterial(GL2.GL_FRONT, GL2.GL_AMBIENT_AND_DIFFUSE);

        //
        gl.glHint(GL2.GL_PERSPECTIVE_CORRECTION_HINT, GL.GL_NICEST);

        // Initialize projection matrix
        reshape(glad, canvas.getBounds().x, canvas.getBounds().y, canvas.getBounds().width, canvas.getBounds().height);

        // setup light
        gl.glEnable(GL2.GL_COLOR_MATERIAL);
        gl.glLightModelfv(GL2.GL_LIGHT_MODEL_AMBIENT, FloatBuffer.wrap(new float[]{0.2f, 0.2f, 0.2f, 1.0f}));
        gl.glLightModelf(GL2.GL_LIGHT_MODEL_LOCAL_VIEWER, GL.GL_TRUE);
        gl.glLightfv(GL2.GL_LIGHT0, GL2.GL_AMBIENT, FloatBuffer.wrap(new float[]{0.5f, 0.5f, 0.5f, 1.0f}));
        gl.glLightfv(GL2.GL_LIGHT0, GL2.GL_DIFFUSE, FloatBuffer.wrap(new float[]{1.0f, 1.0f, 1.0f, 1.0f}));
        gl.glLightfv(GL2.GL_LIGHT0, GL2.GL_SPECULAR, FloatBuffer.wrap(new float[]{1.0f, 1.0f, 1.0f, 1.0f}));
        gl.glLightfv(GL2.GL_LIGHT0, GL2.GL_POSITION, FloatBuffer.wrap(new float[]{0.0f, 0.0f, 9.0f, 1.0f}));
        gl.glLightf(GL2.GL_LIGHT0, GL2.GL_CONSTANT_ATTENUATION, 1.0f);
        gl.glLightf(GL2.GL_LIGHT0, GL2.GL_LINEAR_ATTENUATION, 0.0f);
        gl.glLightf(GL2.GL_LIGHT0, GL2.GL_QUADRATIC_ATTENUATION, 0.0f);
        gl.glLightf(GL2.GL_LIGHT0, GL2.GL_SPOT_CUTOFF, 180.0f);
        gl.glLightfv(GL2.GL_LIGHT0, GL2.GL_SPOT_DIRECTION, FloatBuffer.wrap(new float[]{0.f, 0.f, -1.f}));
        gl.glLightf(GL2.GL_LIGHT0, GL2.GL_SPOT_EXPONENT, 0.0f);
        gl.glEnable(GL2.GL_LIGHT0);

        // Set clear color
        gl.glClearColor(0.3f, 0.3f, 0.35f, 1.0f);

        // Load display elements from scene file
        final Set<String> requiredModels = new HashSet<String>();
        for (final Drawing d : drawings) {
            final Models annotation = d.getClass().getAnnotation(Models.class);
            if (annotation != null) {
                for (final String name : annotation.value()) {
                    requiredModels.add(name);
                }
            }
        }
        try {
            final RoSi2Element scene = RoSi2Element.parseFile("scene/TeamComm.ros2");
            final List<RoSi2Element> elems = scene.findElements(requiredModels);
            for (RoSi2Element elem : elems) {
                objectLists.put(elem.getName(), elem.instantiate(gl).createDisplayList());
            }
        } catch (RoSi2Element.RoSi2ParseException ex) {
            JOptionPane.showMessageDialog(null,
                    ex.getMessage(),
                    "Error loading scene",
                    JOptionPane.ERROR_MESSAGE);
            System.exit(-1);
        } catch (XMLStreamException ex) {
            JOptionPane.showMessageDialog(null,
                    ex.getMessage(),
                    "Error loading scene",
                    JOptionPane.ERROR_MESSAGE);
            System.exit(-1);
        } catch (IOException ex) {
            JOptionPane.showMessageDialog(null,
                    ex.getMessage(),
                    "Error loading scene",
                    JOptionPane.ERROR_MESSAGE);
            System.exit(-1);
        }
    }

    @Override
    public void dispose(final GLAutoDrawable glad) {

    }

    @Override
    public void display(final GLAutoDrawable glad) {
        final GL2 gl = glad.getGL().getGL2();

        // Clear buffers
        gl.glClear(GL.GL_COLOR_BUFFER_BIT | GL.GL_DEPTH_BUFFER_BIT);

        // Position camera
        gl.glLoadIdentity();
        gl.glTranslatef(0, 0, -cameraRadius);
        gl.glRotatef(-cameraTheta, 1, 0, 0);
        gl.glRotatef(cameraPhi, 0, 0, 1);

        // Lock robot states
        RobotData.getInstance().lockForReading();

        // Determine used drawings
        final int[] curTeamNumbers = RobotData.getInstance().getTeamNumbers();
        if (curTeamNumbers == null) {
            if (teamNumbers[0] != PluginLoader.TEAMNUMBER_COMMON || teamNumbers[1] != PluginLoader.TEAMNUMBER_COMMON) {
                drawings.clear();
                teamNumbers[0] = PluginLoader.TEAMNUMBER_COMMON;
                teamNumbers[1] = PluginLoader.TEAMNUMBER_COMMON;
                drawings.addAll(PluginLoader.getInstance().getCommonDrawings());
            }
        } else {
            if (!((curTeamNumbers[0] == teamNumbers[0] && curTeamNumbers[1] == teamNumbers[1])
                    || (curTeamNumbers[0] == teamNumbers[1] && curTeamNumbers[1] == teamNumbers[0]))) {
                drawings.clear();
                teamNumbers[0] = curTeamNumbers[0];
                teamNumbers[1] = curTeamNumbers[1];
                drawings.addAll(PluginLoader.getInstance().getCommonDrawings());
                drawings.addAll(PluginLoader.getInstance().getDrawings(teamNumbers[0]));
                drawings.addAll(PluginLoader.getInstance().getDrawings(teamNumbers[1]));
            }
        }

        // Render drawings
        for (final Drawing d : drawings) {
            if (d.isActive()) {
                if (d instanceof Static) {
                    ((Static) d).draw(gl, objectLists);
                } else if (d instanceof PerPlayer) {
                    if (d.getTeamNumber() == PluginLoader.TEAMNUMBER_COMMON || d.getTeamNumber() == curTeamNumbers[RobotData.TEAM_LEFT]) {
                        for (final Iterator<RobotState> iter = RobotData.getInstance().getRobotsForTeam(RobotData.TEAM_LEFT); iter.hasNext();) {
                            ((PerPlayer) d).draw(gl, objectLists, iter.next(), RobotData.TEAM_LEFT);
                        }
                    }
                    if (d.getTeamNumber() == PluginLoader.TEAMNUMBER_COMMON || d.getTeamNumber() == curTeamNumbers[RobotData.TEAM_RIGHT]) {
                        gl.glRotatef(180, 0, 0, 1);
                        for (final Iterator<RobotState> iter = RobotData.getInstance().getRobotsForTeam(RobotData.TEAM_RIGHT); iter.hasNext();) {
                            ((PerPlayer) d).draw(gl, objectLists, iter.next(), RobotData.TEAM_RIGHT);
                        }
                        gl.glRotatef(180, 0, 0, 1);
                    }
                }
            }
        }

        // Unlock robot states
        RobotData.getInstance().unlockForReading();
    }

    @Override
    public void reshape(final GLAutoDrawable glad, final int x, final int y, final int width, final int height) {
        // Adjust projection matrix
        if (this.width != width || this.height != height) {
            this.width = width;
            this.height = height;
            final GL2 gl = glad.getGL().getGL2();
            final GLU glu = GLU.createGLU(gl);
            gl.glMatrixMode(GL2.GL_PROJECTION);
            gl.glLoadIdentity();
            glu.gluPerspective(40, (double) width / (double) height, NEAR_PLANE, FAR_PLANE);
            gl.glMatrixMode(GL2.GL_MODELVIEW);
        }
    }

    public PriorityQueue<Drawing> getDrawings() {
        return drawings;
    }
}

package controller.ui.ui.components;

import common.TotalScaleLayout;
import controller.action.ActionBoard;
import controller.action.ui.CardIncrease;
import controller.action.ui.MakeGoalieAction;
import controller.net.RobotOnlineStatus;
import controller.net.RobotWatcher;
import controller.ui.helper.FontHelper;
import controller.ui.localization.LocalizationManager;
import controller.ui.ui.customized.Button;
import controller.ui.ui.customized.CountDownBar;
import controller.ui.ui.customized.ImageButton;
import data.Helper;
import data.PlayerInfo;
import data.Rules;
import data.hl.HL;
import data.hl.HLSim;
import data.states.AdvancedData;
import data.values.Penalties;
import data.values.Side;
import teamcomm.Config;

import javax.swing.*;
import javax.swing.border.EmptyBorder;
import java.awt.*;
import java.awt.event.*;

/**
 * Created by rkessler on 2017-06-11.
 */
public class Robot extends AbstractComponent {

    protected Side side;
    protected int id;
    Button robot;
    protected JLabel robotLabel;

    protected JButton warningCard;
    protected JButton yellowCard;
    protected JButton redCard;

    protected JButton makeGoalie;

    private ImageIcon lanIcon;
    private ImageIcon lanOnline;
    private ImageIcon lanHighLatency;
    private ImageIcon lanOffline;
    private ImageIcon lanUnknown;

    private ImageIcon makeGoalieImage;
    private ImageIcon isGoalieImage;

    protected CountDownBar progressBar;

    private static final String ICONS_PATH = "config/icons/";
    private static final String ONLINE = "wlan_status_green.png";
    private static final String OFFLINE = "wlan_status_red.png";

    private static final String IS_GOALIE = "goalkeeper.png";
    private static final String MAKE_GOALIE = "goalkeeper_fade.png";

    private static final String HIGH_LATENCY = "wlan_status_yellow.png";
    private static final String UNKNOWN_ONLINE_STATUS = "wlan_status_grey.png";
    public static final Color COLOR_HIGHLIGHT = Color.YELLOW;

    private static final int UNPEN_HIGHLIGHT_SECONDS = 5;


    public Robot(Side side, int id) {
        this.side = side;
        this.id = id;
        lanOnline = new ImageIcon(Config.CONFIG_ROOT + ICONS_PATH + ONLINE);
        lanHighLatency = new ImageIcon(Config.CONFIG_ROOT + ICONS_PATH + HIGH_LATENCY);
        lanOffline = new ImageIcon(Config.CONFIG_ROOT +  ICONS_PATH + OFFLINE);
        lanUnknown = new ImageIcon(Config.CONFIG_ROOT +  ICONS_PATH + UNKNOWN_ONLINE_STATUS);

        makeGoalieImage = new ImageIcon(Config.CONFIG_ROOT + ICONS_PATH + MAKE_GOALIE);
        isGoalieImage = new ImageIcon(Config.CONFIG_ROOT + ICONS_PATH + IS_GOALIE);

        System.out.println(makeGoalieImage);
        System.out.println(isGoalieImage);
    }


    public void updateLayout(double aspectRatio) {
        TotalScaleLayout robotLayout = new TotalScaleLayout(robot);
        robot.setLayout(robotLayout);
        robot.removeAll();

        // Figure out a way to make this easier
        double rightOffset = 0.01;

        double cardWidth = 0.4 / aspectRatio;
        robotLayout.add(1 - cardWidth - rightOffset, 0.1, cardWidth, 0.75, warningCard);
        robotLayout.add(1 - 2 * cardWidth - rightOffset, 0.1, cardWidth, 0.75, yellowCard);
        robotLayout.add(1 - 3 * cardWidth - rightOffset, 0.1, cardWidth, 0.75, redCard);

        robotLayout.add(1 - 5 * cardWidth - rightOffset, 0.1, cardWidth*2, 0.75, makeGoalie);

        double restWidth = 1 - 2 * cardWidth - rightOffset;

        robotLayout.add(0, 0, restWidth, 0.9, robotLabel);

        robotLayout.add(0, 0.9, 1, 0.1, progressBar);
    }

    public void setup() {
        robot = new Button();
        robot.addActionListener(ActionBoard.robot[side.value()][this.id]);
        robot.addComponentListener(new ComponentAdapter() {
            public void componentResized(ComponentEvent e) {
                // This is only called when the user releases the mouse button.
                double w = e.getComponent().getSize().getWidth();
                double h = e.getComponent().getSize().getHeight();
                double aspect = w / h;
                updateLayout(aspect);
            }
        });

        TotalScaleLayout robotLayout = new TotalScaleLayout(robot);
        robot.setLayout(robotLayout);

        progressBar = new CountDownBar();

        robotLabel = new JLabel();
        robotLabel.setHorizontalAlignment(JLabel.LEFT);
        robotLabel.setIcon(lanIcon);
        robotLabel.setBorder(new EmptyBorder(10, 10, 10, 10));


        warningCard = new Button("Warning");
        warningCard.addActionListener(new CardIncrease(side, this.id, Color.BLUE));

        yellowCard = new Button("Yellow");
        yellowCard.addActionListener(new CardIncrease(side, this.id, Color.YELLOW));

        redCard = new Button("Red");
        redCard.addActionListener(new CardIncrease(side, this.id, Color.RED));

        makeGoalie = new JButton("");
        makeGoalie.addActionListener(new MakeGoalieAction(side, this.id));
        makeGoalie.setIcon(makeGoalieImage);

        makeGoalie.setVisible(true);

        robotLabel.setVisible(true);

        robot.setVisible(true);

        updateLayout(1);

        this.setLayout(new TotalScaleLayout(this));
        ((TotalScaleLayout) this.getLayout()).add(0, 0, 1, 1, robot);
        this.setVisible(true);
    }

    private void updateRobotOnlineStatus() {
        int sideValue = this.side.value();
        int j = this.id;

        //** Online state **/
        RobotOnlineStatus[][] onlineStatus = RobotWatcher.updateRobotOnlineStatus();
        ImageIcon currentLanIcon;
        if (onlineStatus[sideValue][j] == RobotOnlineStatus.ONLINE) {
            currentLanIcon = lanOnline;
        } else if (onlineStatus[sideValue][j] == RobotOnlineStatus.HIGH_LATENCY) {
            currentLanIcon = lanHighLatency;
        } else if (onlineStatus[sideValue][j] == RobotOnlineStatus.OFFLINE) {
            currentLanIcon = lanOffline;
        } else {
            currentLanIcon = lanUnknown;
        }
        robotLabel.setIcon(currentLanIcon);

    }

    @Override
    public void update(AdvancedData data) {
        int sideValue = this.side.value();
        int robotId = this.id;
        PlayerInfo robotInfo = data.team[sideValue].player[robotId];

        // First of all we update the Online Status of the Robot
        updateRobotOnlineStatus();

        // then we update the yellow and red card buttons with the number of cards
        updatePenaltyCards(robotInfo);

        // Update the goalie info
        updateGoalieMarker(robotInfo);

        int seconds = data.getRemainingPenaltyTime(sideValue, robotId);
        boolean isEjected = data.ejected[sideValue][robotId];
        if (robotInfo.penalty != Penalties.NONE &&
                (seconds > 0 || robotInfo.penalty == Penalties.SUBSTITUTE || isEjected)) {
            if (!isEjected) {
                boolean servingPenalty = data.isServingPenalty[sideValue][robotId];
                boolean pickup = (Rules.league instanceof HL || Rules.league instanceof HLSim) && robotInfo.penalty == Penalties.HL_PICKUP_OR_INCAPABLE;

                if (!servingPenalty) {
                    if (pickup) {
                        robotLabel.setText(data.team[sideValue].teamColor + " " + (robotId + 1) + " (" + Penalties.HL_PICKUP_OR_INCAPABLE.toString() + ")");
                        highlight(robot, true);
                    } else if (robotInfo.penalty == Penalties.SUBSTITUTE) {
                        robotLabel.setText(data.team[sideValue].teamColor + " " + (robotId + 1) + " (" + Penalties.SUBSTITUTE.toString() + ")");
                        highlight(robot, false);
                    } else if (Rules.league instanceof HL || Rules.league instanceof HLSim) {
                        robotLabel.setText(data.team[sideValue].teamColor + " " + (robotId + 1) + ": " + Helper.formatTime(seconds));
                        highlight(robot, true);
                    }
                    progressBar.setVisible(false);
                } else {
                    robotLabel.setText(data.team[sideValue].teamColor + " " + (robotId + 1) + ": " + Helper.formatTime(seconds) + (pickup ? " (P)" : ""));
                    highlight(robot, seconds <= UNPEN_HIGHLIGHT_SECONDS && robot.getBackground() != COLOR_HIGHLIGHT);
                    // Update the robot time component
                    int penTime = (seconds + data.getSecondsSince(data.whenPenalized[sideValue][robotId]));
                    double percent = 100.0 * seconds / (double) penTime;
                    progressBar.updateValue(percent);
                }
            } else {
                robotLabel.setText(LocalizationManager.getLocalization().EJECTED);
                highlight(robot, false);
                progressBar.setVisible(false);
            }
        } else {
            robotLabel.setText(data.team[sideValue].teamColor + " " + (robotId + 1));
            highlight(robot, false);
            progressBar.setVisible(false);
        }
    }

    private void updateGoalieMarker(PlayerInfo playerInfo){
        if (playerInfo.isGoalie == 1){
            makeGoalie.setIcon(isGoalieImage);
        } else {
            makeGoalie.setIcon(makeGoalieImage);
        }
    }

    private void updatePenaltyCards(PlayerInfo playerInfo) {
        warningCard.setText(String.valueOf(playerInfo.warningCardCount));
        warningCard.setMargin(new Insets(0, 0, 0, 0));
        warningCard.setFont(FontHelper.boldStandardFont());

        yellowCard.setText(String.valueOf(playerInfo.yellowCardCount));
        yellowCard.setMargin(new Insets(0, 0, 0, 0));
        yellowCard.setFont(FontHelper.boldStandardFont());

        redCard.setText(String.valueOf(playerInfo.redCardCount));
        redCard.setMargin(new Insets(0, 0, 0, 0));
        redCard.setFont(FontHelper.boldStandardFont());

        if (playerInfo.warningCardCount > 0) {
            warningCard.setBackground(new Color(0, 100, 255));
        } else {
            warningCard.setBackground(new Color(50, 180, 255));
        }

        if (playerInfo.yellowCardCount > 0) {
            yellowCard.setBackground(new Color(255, 255, 0));
        } else {
            yellowCard.setBackground(new Color(255, 251, 181));
        }

        if (playerInfo.redCardCount > 0) {
            redCard.setBackground(new Color(255, 0, 0));
        } else {
            redCard.setBackground(new Color(255, 174, 171));
        }
    }


}

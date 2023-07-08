package data.hl;

import data.Rules;
import data.values.Penalties;

import java.awt.Color;

/**
 * This class sets attributes given by the humanoid-league rules.
 *
 * @author Michel-Zen
 */
public class HL extends Rules
{
    public HL()
    {
        /** The league´s name this rules are for. */
        leagueName = "HL Kid";
        /** The league´s directory name with it´s teams and icons. */
        leagueDirectory = "hl_kid";
        /** How many robots are in a team. */
        teamSize = 6; 
        /** How many robots of each team may play at one time. */
        robotsPlaying = 4;
        /** The Java Colors the left and the right team starts with. */
        teamColor = new Color[] {Color.BLUE, Color.RED};
        /** The name of the colors. */
        teamColorName = new String[] {"Blue","Red"};
        /** If the colors change automatically. */
        colorChangeAuto = false;
        /** If the clock may stop in certain states (Ready, Set) in a play-off game. */
        playOffTimeStop = false;
        /** Time in seconds one half is long. */
        halfTime = 10*60;
        /** Time in seconds the ready state is long. */
        readyTime = 45;
        /** Time in seconds between first and second half. */
        pauseTime = 5*60;
        /** Time in seconds between first and second overtime half. */
        pauseTime = 5*60;
        /** If left and right side may both have the first kickoff. */
        kickoffChoice = true;
        /** Time in seconds the ball is blocked after kickoff. */
        kickoffTime = 10;
        /** Time in seconds before a global game stuck can be called. */
        minDurationBeforeStuck = 30;
        /** The number of seconds switching to Playing is delayed. */
        delayedSwitchToPlaying = 0;
        /** If there is an overtime before penalty-shoot in a play-off game. */
        overtime = true;
        /** Time in seconds one overtime half is long. */
        overtimeTime = 5*60;
        /** If the game starts with penalty-shoots. */
        startWithPenalty = false;
        /** Time in seconds between second half and penalty shoot. */
        pausePenaltyShootOutTime = 0;
        /** If there can be a penalty-shoot retry. */
        penaltyShotRetries = true;
        /** Time in seconds one penalty shoot is long. */
        penaltyShotTime = 1*60;
        /** If there is a sudden-death. */
        suddenDeath = false;
        /** Time in seconds one penalty shoot is long in sudden-death. */
        penaltyShotTimeSuddenDeath = 2*60; // does not matter
        /** Number of penalty-shoots for each team when a half has 10minutes. */
        numberOfPenaltyShotsShort = 5;
        /** Number of penalty-shoots for each team after full 10minutes playing. */
        numberOfPenaltyShotsLong = 5;
        /** Time in seconds to increment penalties. */
        penaltyIncreaseTime = 0;
        /** Whether the penalty count is reset on halftime */
        resetPenaltyCountOnHalftime = true;
        /** Whether the ejected robots are reset on halftime */
        resetEjectedRobotsOnHalftime = true;
        /** Whether penalties can be removed before the penalty time has passed. */
        allowEarlyPenaltyRemoval = true;
        /** Penalty that players get when they substitute another player. */
        substitutePenalty = Penalties.NONE;
        /** if robots should return from penalties when the game state changes. */
        returnRobotsInGameStoppages = false;
        /** Time in seconds one team has as timeOut. */
        timeOutTime = 2*60;
        /** One time-out per half? */
        timeOutPerHalf = true;
        /** On how many pushings is a robot ejected. */
        pushesToEjection = new int[] {};
        /** Defines if the option for a referee timeout is available */
        isRefereeTimeoutAvailable = true;
        /** How long the referee timeout is **/
        refereeTimeout = 10*60;
        /** Defines if coach is available */
        isCoachAvailable = false;
        /** Allowed to compensate for lost time? */
        lostTime = false;
        /** If true, the drop-in player competition is active */
        dropInPlayerMode = false;
        /** If true, the game controller should drop broadcast-messages */
        dropBroadcastMessages = false;
        /** Background Side **/
        backgroundSide = new String[][]{
                {
                        "robot_left_blue.png",
                        "robot_left_red.png",
                },
                {
                        "robot_right_blue.png",
                        "robot_right_red.png",
                }
        };

        /** The time a team has to prepare for the free kick **/
        game_interruption_preparation_time = 30;

        /** The time a team has to prepare for the penalty kick **/
        penalty_kick_preparation_time = 30;

        /** The time an opponent is guaranteed to move away from the ball in preparation phase **/
        game_interruption_minimal_ready_time = 15;

        /** In Playoffs, it can be enabled to make up for the READY and SET time by adding them to the game for each time the phase was entered**/
        enableAddingTimeInCurrentStateForPlayoffs = false;
    }
}

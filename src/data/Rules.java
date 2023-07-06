package data;

import data.hl.*;
import data.values.Penalties;

import java.awt.Color;


/**
 * This class holds attributes defining rules.
 * 
 * @author Michel Bartsch
 */
public abstract class Rules
{   
    /** Note all league´s rules here to have them available. */
    public static final Rules[] LEAGUES = {
        new HL(),
        new HLTeen(),
        new HLAdult(),
        new HLSimulationKid(),
        new HLSimulationAdult(),
        new HLDropIn3(),
        new HLDropIn5()
    };

    public boolean equals(Rules rules) {
        if (this.getClass().getSimpleName().equals(rules.getClass().getSimpleName())) {
            return true;
        }
        return false;
    }

    /** The rules of the league playing. */
    public static Rules league = LEAGUES[0];

    /** The league´s name this rules are for. */
    public String leagueName;
    /** The league´s directory name with it´s teams and icons. */
    public String leagueDirectory;
    /** How many robots are in a team. */
    public int teamSize;
    /** How many robots of each team may play at one time. */
    public int robotsPlaying;
    /** The Java Colors the left and the right team starts with. */
    public Color[] teamColor;
    /** The name of the colors. */
    public String[] teamColorName;
    /** If the colors change automatically. */
    public boolean colorChangeAuto;
    /** If the clock may stop in certain states (Ready, Set) in a play-off game. */
    public boolean playOffTimeStop;
    /** Time in seconds one half is long. */
    public int halfTime;
    /** Time in seconds the ready state is long. */
    public int readyTime;
    /** Time in seconds between first and second half. */
    public int pauseTime;
    /** Time in seconds between first and second overtime half. */
    public int pauseOverTime;
    /** If left and right side may both have the first kickoff. */
    public boolean kickoffChoice;
    /** Time in seconds the ball is blocked after kickoff. */
    public int kickoffTime;
    /** Time in seconds the ball is blocked after a game interruption was executed. */
    public int blockedAfterGameInterruption;
    /** Time in seconds before a global game stuck can be called. */
    public int minDurationBeforeStuck;
    /** The number of seconds switching to Playing is delayed. */
    public int delayedSwitchToPlaying;
    /** If there is an overtime before the penalty shoot-out in a play-off game. */
    public boolean overtime;
    /** Time in seconds one overtime half is long. */
    public int overtimeTime;
    /** If the game starts with penalty-shots. */
    public boolean startWithPenalty;
    /** Time in seconds between second half and penalty shoot-out. */
    public int pausePenaltyShootOutTime;
    /** Time in seconds one penalty shoot is long. */
    public int penaltyShotTime;
    /** If there can be a penalty-shot retry. */
    public boolean penaltyShotRetries;
    /** If there is a sudden-death. */
    public boolean suddenDeath;
    /** Time in seconds one penalty shoot is long in sudden-death. */
    public int penaltyShotTimeSuddenDeath;
    /** Number of penalty-shots for each team when a half has 10minutes. */
    public int numberOfPenaltyShotsShort;
    /** Number of penalty-shots for each team after full 10minutes playing. */
    public int numberOfPenaltyShotsLong;
    /** Time in seconds to increment penalties. */
    public int penaltyIncreaseTime; 
    /** Whether the penalty count is reset on halftime */
    public boolean resetPenaltyCountOnHalftime;
    /** Whether the ejected robots are reset on halftime */
    public boolean resetEjectedRobotsOnHalftime;
    /** Whether penalties can be removed before the penalty time has passed. */
    public boolean allowEarlyPenaltyRemoval;
    /** Penalty that players get when they substitute another player. */
    public Penalties substitutePenalty;
    /** if robots should return from penalties when the game state changes. */
    public boolean returnRobotsInGameStoppages;
    /** Time in seconds one team has as timeOut. */
    public int timeOutTime;
    /** Time in seconds of a referee timeout */
    public int refereeTimeout;
    /** Defines if the option for a referee timeout is available. */ 
    public boolean isRefereeTimeoutAvailable;
    /** One time-out per half? */
    public boolean timeOutPerHalf;
    /** On how many pushings is a robot ejected. */
    public int[] pushesToEjection;
    /** Defines if coach is available */
    public boolean isCoachAvailable;
    /** Allowed to compensate for lost time? */
    public boolean lostTime;
    /** If true, the drop-in player competition is active */
    public boolean dropInPlayerMode;
    /** If true, the game controller should drop broadcast-messages */
    public boolean dropBroadcastMessages;
    /** BackgroundSide **/
    public String[][] backgroundSide;

    /** The time a team has to prepare for game_interruption (free kick, corner kicks, etc...)
     * @see controller.action.ui.GameInterruption */
    public int game_interruption_preparation_time;

    /** The time a team has to prepare for the penalty kick **/
    public int penalty_kick_preparation_time;

    /** The time an opponent is guaranteed to move away from the ball in preparation phase **/
    public int game_interruption_minimal_ready_time;

    /** In Playoffs, it can be enabled to make up for the READY and SET time by adding them to the game for each time the phase was entered**/
    public boolean enableAddingTimeInCurrentStateForPlayoffs;


}
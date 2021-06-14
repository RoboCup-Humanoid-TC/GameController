package controller.net;

import common.Log;
import data.states.AdvancedData;
import data.communication.GameControlData;
import data.Teams;
import data.values.GameStates;
import data.values.GameTypes;

import java.io.IOException;
import java.net.*;
import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReadWriteLock;
import java.util.concurrent.locks.ReentrantReadWriteLock;

/**
 * @author Marcel Steinbeck
 *
 * This class is used to send the current {@link GameControlData} (game-state)
 * to all robots every 500 ms. The package will be send via UDP on port
 * {@link GameControlData#GAMECONTROLLER_GAMEDATA_PORT} over broadcast.
 *
 * To prevent race-conditions (the sender is executed in its thread-context),
 * the sender will hold a deep copy of {@link GameControlData} (have a closer
 * look to the copy-constructor
 *
 * This class is a singleton!
 */
public class Sender extends Thread {

    /**
     * The instance of the singleton.
     */
    private static Sender instance;

    /**
     * The packet number that is increased with each packet sent.
     */
    private byte packetNumber = 0;

    /**
     * The socket, which is used to send the current game-state
     */
    private final DatagramSocket datagramSocket;

    /**
     * The used inet-address (the broadcast address).
     */
    private final InetAddress group;

    /**
     * The current deep copy of the game-state.
     */
    private AdvancedData data;

    private final ReadWriteLock readWriteLockData = new ReentrantReadWriteLock();
    private final Lock readLockData = readWriteLockData.readLock();
    private final Lock writeLockData = readWriteLockData.writeLock();

    /**
     *
     */
    public static int UPDATEFREQUENCY = 500;

    /**
     * Creates a new Sender.
     *
     * @throws SocketException if an error occurs while creating the socket
     * @throws UnknownHostException if the used inet-address is not valid
     */
    private Sender(final InetAddress broadcastAddress) throws SocketException, UnknownHostException {
        instance = this;

        this.datagramSocket = new DatagramSocket();
        this.group = broadcastAddress;
    }

    /**
     * Initialises the Sender. This needs to be called before
     * {@link #getInstance()} is available.
     *
     * @param broadcastAddress the broadcast address to use
     * @throws SocketException if an error occurs while creating the socket
     * @throws UnknownHostException if the used inet-address is not valid
     * @throws IllegalStateException if the sender is already initialized
     */
    public synchronized static void initialize(final InetAddress broadcastAddress) throws SocketException, UnknownHostException {
        System.out.println("Initializing Broadcasting server on " + broadcastAddress);
        if (null != instance) {
            throw new IllegalStateException("sender is already initialized");
        } else {
            instance = new Sender(broadcastAddress);
        }
    }

    /**
     * Returns the instance of the singleton.
     *
     * @return The instance of the Sender
     * @throws IllegalStateException if the Sender is not initialized yet
     */
    public synchronized static Sender getInstance() {
        if (null == instance) {
            throw new IllegalStateException("sender is not initialized yet");
        } else {
            return instance;
        }
    }

    /**
     * Sets the current game-state to send. Creates a deep copy of data to
     * prevent race-conditions. Have a closer look to
     *
     * @param data the current game-state to send to all robots
     */
    public void send(AdvancedData data) {
        // Adjust name of the current teamcomm logfile
        if (this.data == null
                || (data.gameState == GameStates.READY && this.data.gameState == GameStates.INITIAL)
                || (data.gameState == GameStates.INITIAL && this.data.gameState != GameStates.INITIAL)
                || (data.gameState == GameStates.FINISHED && this.data.gameState != GameStates.FINISHED)) {
            final StringBuilder logfileName;
            final String[] teamNames = Teams.getNames(false);
            if (data.gameType == GameTypes.DROPIN) {
                logfileName = new StringBuilder("Drop-in_");
                if (data.firstHalf == GameControlData.C_TRUE) {
                    logfileName.append("1st");
                } else {
                    logfileName.append("2nd");
                }
                logfileName.append("Half");
            } else {
                if (data.firstHalf == GameControlData.C_TRUE) {
                    logfileName = new StringBuilder(data.team[0].teamNumber < teamNames.length && teamNames[data.team[0].teamNumber] != null ? teamNames[data.team[0].teamNumber] : "Unknown").append("_").append(data.team[1].teamNumber < teamNames.length && teamNames[data.team[1].teamNumber] != null ? teamNames[data.team[1].teamNumber] : "Unknown").append("_1st");
                } else {
                    logfileName = new StringBuilder(data.team[1].teamNumber < teamNames.length && teamNames[data.team[1].teamNumber] != null ? teamNames[data.team[1].teamNumber] : "Unknown").append("_").append(data.team[0].teamNumber < teamNames.length && teamNames[data.team[0].teamNumber] != null ? teamNames[data.team[0].teamNumber] : "Unknown").append("_2nd");
                }
                logfileName.append("Half");
            }
            if (data.gameState == GameStates.READY && (this.data == null || this.data.gameState == GameStates.INITIAL)) {
                teamcomm.net.logging.Logger.getInstance().createLogfile(logfileName.toString());
            } else if (data.gameState == GameStates.INITIAL && (this.data == null || this.data.gameState != GameStates.INITIAL)) {
                teamcomm.net.logging.Logger.getInstance().createLogfile(logfileName.append("_initial").toString());
            } else if (data.gameState == GameStates.FINISHED && (this.data == null || this.data.gameState != GameStates.FINISHED)) {
                teamcomm.net.logging.Logger.getInstance().createLogfile(logfileName.append("_finished").toString());
            }
        }

        writeLockData.lock();
        try {
            // Clone data
            this.data = (AdvancedData) data.clone();
        } finally {
            writeLockData.unlock();
        }
    }

    @Override
    public void run() {
        while (!isInterrupted()) {
            readLockData.lock();
            try {
                if (data != null) {
                    data.updateTimes();
                    data.packetNumber = packetNumber;
                    teamcomm.net.logging.Logger.getInstance().log(data);
                    byte[] arr = data.toByteArray().array();
                    DatagramPacket packet = new DatagramPacket(arr, arr.length, group, data.GAMECONTROLLER_GAMEDATA_PORT);

                    try {
                        datagramSocket.send(packet);
                        packetNumber++;
                    } catch (IOException e) {
                        Log.error("Error while sending");
                        e.printStackTrace();
                    }
                    data = null;
                }
            } finally {
                readLockData.unlock();
            }

            try {
                Thread.sleep(UPDATEFREQUENCY);
            } catch (InterruptedException e) {
                interrupt();
            }
        }

        datagramSocket.close();
    }
}

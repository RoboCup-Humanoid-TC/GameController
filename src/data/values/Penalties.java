package data.values;

/**
 * Created by rkessler on 2017-03-25.
 */
public enum Penalties implements DocumentingMarkdown {

    UNKNOWN(255, "Unknown", -1),

    NONE(0, "None", -1),

    SUBSTITUTE(14, "substitute", -1), // TODO check if different for SPL than HL and what value is
    MANUAL(15, "manual", -1), // TODO check if different for SPL than HL and what value is

    HL_BALL_MANIPULATION(30, "ball_manipulation", 10),
    HL_PHYSICAL_CONTACT(31, "pushing", 10),
    HL_PICKUP_OR_INCAPABLE(34, "pickup/incapable", 10),
    HL_SERVICE(35, "service", 60);

    private byte byte_value;
    private String humanReadable;
    private int penaltyTime;

    Penalties(int byte_value, String humanReadable, int penalty_time) {
        this.byte_value = (byte) byte_value;
        this.humanReadable = humanReadable;
        this.penaltyTime = penalty_time;
    }

    public byte value() {
        return byte_value;
    }

    public int penaltyTime(){
        return this.penaltyTime;
    }
    public static Penalties fromValue(byte b) {
        for(Penalties penalty : Penalties.values()){
            if (penalty.value() == b){
                return penalty;
            }
        }
        System.out.println("Warning: Could not resolve Penalty byte.");
        return UNKNOWN;
    }

    public String toString(){
        return this.humanReadable;
    }
}

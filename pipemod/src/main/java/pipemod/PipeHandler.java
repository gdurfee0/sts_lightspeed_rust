package pipemod;

import basemod.BaseMod;
import basemod.interfaces.PostInitializeSubscriber;
import basemod.interfaces.StartGameSubscriber;
import pipemod.message.Gold;
import pipemod.message.Notification;

import com.fasterxml.jackson.databind.ObjectMapper;

import java.io.IOException;
import java.io.RandomAccessFile;
import java.nio.charset.StandardCharsets;

import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

import com.megacrit.cardcrawl.dungeons.AbstractDungeon;

public class PipeHandler implements PostInitializeSubscriber, StartGameSubscriber {
    private static final String STS_TO_TELEMETRY_PIPE_NAME = "\\\\.\\pipe\\sts2telemetry";
    private static final Logger logger = LogManager.getLogger(PipeMod.modID);

    private RandomAccessFile pipe;

    public PipeHandler() {
        BaseMod.subscribe(this);
    }

    private void connectPipe() {
        try {
            pipe = new RandomAccessFile(STS_TO_TELEMETRY_PIPE_NAME, "rw");
        } catch (IOException e) {
            System.err.println("Error connecting to the named pipe: " + e.getMessage());
        }
    }

    @Override
    public void receiveStartGame() {
        String toSend = "Player has " + AbstractDungeon.player.gold + " gold";
        try {
            Gold gold = new Gold();
            gold.gold = AbstractDungeon.player.gold;

            // Serialize Notification to JSON
            ObjectMapper objectMapper = new ObjectMapper();
            String jsonMessage = objectMapper.writeValueAsString((Notification) gold);

            // Send the JSON message to the Rust server
            pipe.write(jsonMessage.getBytes(StandardCharsets.UTF_8));
        } catch (IOException e) {
            System.err.println("Error writing to the named pipe: " + e.getMessage());
        }
    }

    @Override
    public void receivePostInitialize() {
        connectPipe();
        logger.info("PipeHandler initialized", pipe);
    }
}
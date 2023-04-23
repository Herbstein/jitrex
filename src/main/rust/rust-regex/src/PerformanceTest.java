package com.humio.jitrex;

import java.util.*;

public class PerformanceTest {
    private static abstract class RegexpBackend {
        public abstract int countMatches(List<String> inputs);
    }

    private static class RustRegexBackend extends RegexpBackend {
        private static native int internalCountMatches(String pattern, String[] inputs);

        static {
            System.loadLibrary("regex-jni");
        }

        @Override
        public int countMatches(List<String> inputs) {
            return 0;
        }
    }
}
